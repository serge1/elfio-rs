/*
Copyright (C) 2021-present by Serge Lamikhov-Center

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

/*
   Copyright 2021 Serge Lamikhov-Center

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use std::io;

use super::header::*;
use super::section::*;
use super::segment::*;
use super::strings::*;
use super::types::*;
use super::utils::*;

/// Elfio - the main struct of the library. All access to ELF files attributes
/// starts from this object.
/// The object provides functions to access ELF file header attributes as well
/// as the list of segments and sections of this file.

// --------------------------------------------------------------------------
pub struct Elfio {
    header:    Box<dyn ElfHeaderTrait>,
    converter: Converter,
    sections:  Vec<Box<dyn ElfSectionTrait>>,
    segments:  Vec<Box<dyn ElfSegmentTrait>>,
}

// --------------------------------------------------------------------------
impl Elfio {
    /// Create a new instance
    pub fn new() -> Self {
        Elfio {
            converter: Converter { is_needed: false },
            header:    Box::new(ElfHeader::<Elf64Addr, Elf64Off>::new()),
            sections:  Vec::new(),
            segments:  Vec::new(),
        }
    }

    /// Create a new instance with defined encoding and endianess
    pub fn new_(encoding: u8, endianess: u8) -> Self {
        Elfio {
            converter: if (endianess == constant::ELFDATA2LSB && cfg!(target_endian = "little"))
                || endianess == constant::ELFDATA2MSB && cfg!(target_endian = "big")
            {
                Converter { is_needed: false }
            } else {
                Converter { is_needed: true }
            },
            header:    if encoding == constant::ELFCLASS64 {
                Box::new(ElfHeader::<Elf64Addr, Elf64Off>::new())
            } else {
                Box::new(ElfHeader::<Elf32Addr, Elf32Off>::new())
            },
            sections:  Vec::new(),
            segments:  Vec::new(),
        }
    }

    /// Returns a reference for an endianess converter used for the current file
    pub fn get_converter(&self) -> &Converter {
        &self.converter
    }

    /// Load the ELF file from input stream
    pub fn load(&mut self, reader: &mut (dyn ElfioReadSeek)) -> io::Result<()> {
        let mut e_ident: [u8; constant::EI_NIDENT] = [0; constant::EI_NIDENT];
        // Read ELF file signature
        reader.read_exact(&mut e_ident)?;
        reader.seek(io::SeekFrom::Start(0))?;

        // Is it ELF file?
        if e_ident[constant::EI_MAG0] != constant::ELFMAG0
            || e_ident[constant::EI_MAG1] != constant::ELFMAG1
            || e_ident[constant::EI_MAG2] != constant::ELFMAG2
            || e_ident[constant::EI_MAG3] != constant::ELFMAG3
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File signature doesn't conform ELF file",
            ));
        }

        if e_ident[constant::EI_CLASS] != constant::ELFCLASS64
            && e_ident[constant::EI_CLASS] != constant::ELFCLASS32
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown ELF class value",
            ));
        }

        if e_ident[constant::EI_DATA] != constant::ELFDATA2LSB
            && e_ident[constant::EI_DATA] != constant::ELFDATA2MSB
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown ELF file endianess",
            ));
        }

        if e_ident[constant::EI_CLASS] == constant::ELFCLASS64 {
            self.header = Box::new(ElfHeader::<Elf64Addr, Elf64Off>::new());
        } else {
            self.header = Box::new(ElfHeader::<Elf32Addr, Elf32Off>::new());
        }

        if (cfg!(target_endian = "little") && (e_ident[constant::EI_DATA] == constant::ELFDATA2LSB))
            || (cfg!(target_endian = "big")
                && (e_ident[constant::EI_DATA] == constant::ELFDATA2MSB))
        {
            self.converter.is_needed = false;
        } else {
            self.converter.is_needed = true;
        }
        self.header.set_converter(&self.converter);

        match self.header.load(reader) {
            Ok(_) => (),
            Err(e) => return Err(e),
        };

        self.load_sections(reader)?;
        self.load_segments(reader)?;

        Ok(())
    }

    /// Retrieve all ELF file sections
    pub fn get_sections(&self) -> &Vec<Box<dyn ElfSectionTrait>> {
        &self.sections
    }

    /// Retrieve all ELF file segments
    pub fn get_segments(&self) -> &Vec<Box<dyn ElfSegmentTrait>> {
        &self.segments
    }

    /// Retrieve ELF file section by its name
    pub fn get_section_by_name(&self, section_name: &str) -> Option<&dyn ElfSectionTrait> {
        for section in &self.sections {
            if section.get_name() == section_name {
                return Some(&**section);
            }
        }

        None
    }

    /// Retrieve ELF file section by its index
    pub fn get_section_by_index(&self, index: ElfHalf) -> Option<&dyn ElfSectionTrait> {
        let index = index as usize;
        if index < self.sections.len() {
            return Some(&*self.sections[index]);
        }

        None
    }

    fn load_sections(&mut self, reader: &mut (dyn ElfioReadSeek)) -> io::Result<()> {
        let entry_size = self.header.get_section_entry_size() as Elf64Off;
        let num = self.header.get_sections_num() as Elf64Off;
        let offset = self.header.get_sections_offset();

        for i in 0..num {
            let mut section = self.create_section();
            reader.seek(io::SeekFrom::Start(i * entry_size + offset))?;
            section.load(reader)?;
            self.sections.push(section);
        }

        let shstrndx = self.get_section_name_str_index();
        if shstrndx != constant::SHN_UNDEF {
            for i in 1..num {
                let pos = self.sections[i as usize].get_name_string_offset();
                let acc = StringSectionAccessor::new(self, &*self.sections[shstrndx as usize]);
                let name = acc.get_string(pos);
                self.sections[i as usize].set_name(&name);
            }
        }

        Ok(())
    }

    fn create_section(&self) -> Box<dyn ElfSectionTrait> {
        let section: Box<dyn ElfSectionTrait> = if self.header.get_class() == constant::ELFCLASS64 {
            Box::new(ElfSection::<Elf64Addr, Elf64Off, ElfXword>::new(
                &self.converter,
            ))
        } else {
            Box::new(ElfSection::<Elf32Addr, Elf32Off, ElfWord>::new(
                &self.converter,
            ))
        };

        section
    }

    fn load_segments(&mut self, reader: &mut (dyn ElfioReadSeek)) -> io::Result<()> {
        let entry_size = self.header.get_segment_entry_size() as Elf64Off;
        let num = self.header.get_segments_num() as Elf64Off;
        let offset = self.header.get_segments_offset();

        for i in 0..num {
            let mut segment = self.create_segment();
            reader.seek(io::SeekFrom::Start(i * entry_size + offset))?;
            segment.load(reader)?;
            self.segments.push(segment);
        }

        Ok(())
    }

    fn create_segment(&self) -> Box<dyn ElfSegmentTrait> {
        let segment: Box<dyn ElfSegmentTrait> = if self.header.get_class() == constant::ELFCLASS64 {
            Box::new(ElfSegment::<Elf64Addr, Elf64Off, ElfXword>::new(
                &self.converter,
                self.header.get_class(),
            ))
        } else {
            Box::new(ElfSegment::<Elf32Addr, Elf32Off, ElfWord>::new(
                &self.converter,
                self.header.get_class(),
            ))
        };

        segment
    }

    ELFIO_HEADER_ACCESS_GET!(u8, class);
    ELFIO_HEADER_ACCESS_GET!(u8, elf_version);
    ELFIO_HEADER_ACCESS_GET!(u8, encoding);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, header_size);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, section_entry_size);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, segment_entry_size);

    ELFIO_HEADER_ACCESS_GET_SET!(ElfWord, version);
    ELFIO_HEADER_ACCESS_GET_SET!(u8, os_abi);
    ELFIO_HEADER_ACCESS_GET_SET!(u8, abi_version);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, type);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, machine);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfWord, flags);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Addr, entry);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, sections_num);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Off, sections_offset);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, segments_num);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Off, segments_offset);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, section_name_str_index);
}

impl std::fmt::Debug for Elfio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elfio")
            //  .field("x", &self.x)
            //  .field("y", &self.y)
            .finish()
    }
}

impl Default for Elfio {
    fn default() -> Self {
        Self::new()
    }
}
