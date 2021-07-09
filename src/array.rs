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

use super::utils::Convert;
use super::*;
use std::convert::TryFrom;

/// An array element
///
/// See documentation for [ArraySectionAccessor] for usage example
#[derive(Debug, Default)]
pub struct Array {
    /// An array entry
    pub value: Elf64Addr,
}

#[repr(C)]
#[derive(Default)]
struct Elf32Array {
    pub value: Elf32Addr,
}

#[repr(C)]
#[derive(Default)]
struct Elf64Array {
    pub value: Elf64Addr,
}

/// A section data accessor intended to array tables. The accessor is useful
/// for manipulation of such sections as .ctors, .dtors, .init_array and .fini_array
///
/// For example:
/// ```
/// use std::fs::File;
/// use std::io;
/// use std::io::{BufReader, Error};
///
/// use elfio::Elfio;
///
/// fn main() -> io::Result<()> {
///     let elf_file = File::open("tests/files/hello_32")?;
///     let mut reader = BufReader::new(elf_file);
/// 
///     let mut elf = Elfio::new();
/// 
///     elf.load(&mut reader)?;
/// 
///     let section = match elf.get_section_by_name(&".ctors") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
/// 
///     let array = elfio::ArraySectionAccessor::new(&elf, section);
/// 
///     assert_eq!(array.get_entries_num(), 2);
/// 
///     let element = array.get_entry(0).unwrap();
///     assert_eq!(element.value, 0xFFFFFFFF);
///     let element = array.get_entry(1).unwrap();
///     assert_eq!(element.value, 0x00000000);
///
///     Ok(())
/// }
/// ```
pub struct ArraySectionAccessor<'a> {
    elfio:   &'a Elfio,
    section: &'a dyn ElfSectionTrait,
}

impl<'a> ArraySectionAccessor<'a> {
    /// Creates a new instance of the relocation table accessor
    pub fn new(elfio: &'a Elfio, section: &'a dyn ElfSectionTrait) -> ArraySectionAccessor<'a> {
        ArraySectionAccessor { elfio, section }
    }

    /// Returns number of symbols
    pub fn get_entries_num(&self) -> ElfXword {
        let entry_size;
        if self.elfio.get_class() == constant::ELFCLASS64 {
            entry_size = 8;
        }
        else {
            entry_size = 4;
        }

        if entry_size != 0 {
            return self.section.get_size() / entry_size;
        }

        0
    }

    /// Get a relocation entry by its index
    pub fn get_entry(&self, index: ElfXword) -> Option<Array> {
        let entries_num = self.get_entries_num();
        if entries_num == 0 || index > entries_num - 1 {
            return None;
        }

        let entry_size;
        if self.elfio.get_class() == constant::ELFCLASS64 {
            entry_size = 8;
        }
        else {
            entry_size = 4;
        }

        let offset: usize = (index * entry_size) as usize;
        let end: usize = offset + entry_size as usize;
        let entry_area = &self.section.get_data()[offset..end];

        let converter = self.elfio.get_converter();

        if self.elfio.get_class() == constant::ELFCLASS64 {
            let mut entry: Elf64Array = Default::default();
            entry.value = converter.convert(u64::from_ne_bytes(
                <[u8; 8]>::try_from(&entry_area[0..8])
                    .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            ));

            Some(Array { value: entry.value })
        } else {
            let mut entry: Elf32Array = Default::default();
            entry.value = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&entry_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));

            Some(Array {
                value: entry.value as Elf64Addr,
            })
        }
    }
}
