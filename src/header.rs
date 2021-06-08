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

extern crate num;

use super::utils::*;
use super::*;
use num::{cast::AsPrimitive, Zero};
use std::fs::File;
use std::io::BufReader;

// --------------------------------------------------------------------------
pub trait ElfHeaderAccessTrait {
    ELFIO_GET_ACCESS_DECL!(u8, class);
    ELFIO_GET_ACCESS_DECL!(u8, elf_version);
    ELFIO_GET_ACCESS_DECL!(u8, encoding);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, header_size);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, section_entry_size);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, segment_entry_size);

    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, version);
    ELFIO_GET_SET_ACCESS_DECL!(u8, os_abi);
    ELFIO_GET_SET_ACCESS_DECL!(u8, abi_version);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, type);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, machine);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, flags);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, entry);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, sections_num);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, sections_offset);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, segments_num);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, segments_offset);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, section_name_str_index);

    fn set_converter(&mut self, converter: &Converter);
}

// --------------------------------------------------------------------------
pub trait ElfHeaderTrait: ElfHeaderAccessTrait + Load {}

// --------------------------------------------------------------------------
// ELF file header
#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader<Addr, Offset> {
    e_ident: [u8; EI_NIDENT],
    e_type: ElfHalf,
    e_machine: ElfHalf,
    e_version: ElfWord,
    e_entry: Addr,
    e_phoff: Offset,
    e_shoff: Offset,
    e_flags: ElfWord,
    e_ehsize: ElfHalf,
    e_phentsize: ElfHalf,
    e_phnum: ElfHalf,
    e_shentsize: ElfHalf,
    e_shnum: ElfHalf,
    e_shstrndx: ElfHalf,

    converter: Converter,
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfHeader<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    pub fn new() -> ElfHeader<Addr, Offset> {
        ElfHeader::<Addr, Offset> {
            converter: Converter { is_needed: false },

            e_ident: [
                ELFMAG0,
                ELFMAG1,
                ELFMAG2,
                ELFMAG3,
                ELFCLASSNONE,
                ELFDATANONE,
                EV_NONE,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: Addr::zero(),
            e_phoff: Offset::zero(),
            e_shoff: Offset::zero(),
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfHeaderTrait for ElfHeader<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfHeaderAccessTrait for ElfHeader<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    ELFIO_GET_ACCESS!(u8, class, e_ident[EI_CLASS]);
    ELFIO_GET_ACCESS!(u8, elf_version, e_ident[EI_VERSION]);
    ELFIO_GET_ACCESS!(u8, encoding, e_ident[EI_DATA]);
    ELFIO_GET_ACCESS!(ElfHalf, header_size, e_ehsize);
    ELFIO_GET_ACCESS!(ElfHalf, section_entry_size, e_shentsize);
    ELFIO_GET_ACCESS!(ElfHalf, segment_entry_size, e_phentsize);

    ELFIO_GET_SET_ACCESS!(ElfWord, version, e_version);
    ELFIO_GET_SET_ACCESS!(u8, os_abi, e_ident[EI_OSABI]);
    ELFIO_GET_SET_ACCESS!(u8, abi_version, e_ident[EI_ABIVERSION]);
    ELFIO_GET_SET_ACCESS!(ElfHalf, type, e_type);
    ELFIO_GET_SET_ACCESS!(ElfHalf, machine, e_machine);
    ELFIO_GET_SET_ACCESS!(ElfWord, flags, e_flags);
    ELFIO_GET_SET_ACCESS!(ElfHalf, section_name_str_index, e_shstrndx);
    ELFIO_GET_SET_ACCESS!(Elf64Addr, entry, e_entry);
    ELFIO_GET_SET_ACCESS!(ElfHalf, sections_num, e_shnum);
    ELFIO_GET_SET_ACCESS!(Elf64Off, sections_offset, e_shoff);
    ELFIO_GET_SET_ACCESS!(ElfHalf, segments_num, e_phnum);
    ELFIO_GET_SET_ACCESS!(Elf64Off, segments_offset, e_phoff);

    fn set_converter(&mut self, converter: &Converter) {
        self.converter = *converter;
    }
}

// --------------------------------------------------------------------------
impl<Addr, Offset> Load for ElfHeader<Addr, Offset>
where
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    fn load(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
        (&mut self.e_ident).load(reader)?;
        self.e_type.load(reader)?;
        self.e_machine.load(reader)?;
        self.e_version.load(reader)?;
        self.e_entry.load(reader)?;
        self.e_phoff.load(reader)?;
        self.e_shoff.load(reader)?;
        self.e_flags.load(reader)?;
        self.e_ehsize.load(reader)?;
        self.e_phentsize.load(reader)?;
        self.e_phnum.load(reader)?;
        self.e_shentsize.load(reader)?;
        self.e_shnum.load(reader)?;
        self.e_shstrndx.load(reader)?;

        self.e_type = self.converter.convert(self.e_type);
        self.e_machine = self.converter.convert(self.e_machine);
        self.e_version = self.converter.convert(self.e_version);
        self.e_entry = self.converter.convert(self.e_entry);
        self.e_phoff = self.converter.convert(self.e_phoff);
        self.e_shoff = self.converter.convert(self.e_shoff);
        self.e_flags = self.converter.convert(self.e_flags);
        self.e_ehsize = self.converter.convert(self.e_ehsize);
        self.e_phentsize = self.converter.convert(self.e_phentsize);
        self.e_phnum = self.converter.convert(self.e_phnum);
        self.e_shentsize = self.converter.convert(self.e_shentsize);
        self.e_shnum = self.converter.convert(self.e_shnum);
        self.e_shstrndx = self.converter.convert(self.e_shstrndx);

        Ok(())
    }
}
