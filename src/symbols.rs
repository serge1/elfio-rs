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

use std::convert::TryFrom;

use super::elfio::*;
use super::section::*;
use super::strings::*;
use super::types::*;
use super::utils::Convert;

// --------------------------------------------------------------------------
/// A struct represents a single symbol from symbol table section
///
/// See documentation for [SymbolSectionAccessor] for usage example
#[derive(Debug, Default)]
pub struct Symbol {
    /// The name of the associated symbol
    pub name:  String,
    /// The value of the associated symbol
    pub value: Elf64Addr,
    /// The symbol's associated size
    pub size:  ElfXword,
    /// This member specifies the symbol's binding attribute
    pub bind:  u8,
    /// This member specifies the symbol's type attribute
    pub stype: u8,
    /// This member specifies a symbol's visibility
    pub other: u8,
    /// Every symbol table entry is defined in relation to some section.
    /// This member holds the relevant section header table index.
    pub shndx: ElfHalf,
}

// --------------------------------------------------------------------------
#[repr(C)]
#[derive(Default)]
struct Elf32Sym {
    st_name:  ElfWord,
    st_value: Elf32Addr,
    st_size:  ElfWord,
    st_info:  u8,
    st_other: u8,
    st_shndx: ElfHalf,
}

#[repr(C)]
#[derive(Default)]
struct Elf64Sym {
    st_name:  ElfWord,
    st_info:  u8,
    st_other: u8,
    st_shndx: ElfHalf,
    st_value: Elf64Addr,
    st_size:  ElfXword,
}

// --------------------------------------------------------------------------
/// A section data accessor intended to symbol tables
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
///     let section = match elf.get_section_by_name(&".symtab") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
///
///     let symtab = elfio::SymbolSectionAccessor::new(&elf, &*section);
///     assert_eq!(symtab.get_symbols_num(), 0x44);
///     // Num:    Value  Size Type    Bind   Vis      Ndx Name
///     //  30: 08049588     4 OBJECT  LOCAL  DEFAULT   23 dtor_idx.5805
///     let sym = symtab.get_symbol(30).unwrap();
///     assert_eq!(sym.value, 0x08049588);
///     assert_eq!(sym.size, 4);
///     assert_eq!(sym.bind, elfio::constant::STB_LOCAL);
///     assert_eq!(sym.stype, elfio::constant::STT_OBJECT);
///     assert_eq!(sym.shndx, 23);
///     assert_eq!(sym.name, "dtor_idx.5805");
///
///     Ok(())
/// }
/// ```
pub struct SymbolSectionAccessor<'a> {
    elfio:   &'a Elfio,
    section: &'a dyn ElfSectionTrait,
}

// --------------------------------------------------------------------------
impl<'a> SymbolSectionAccessor<'a> {
    /// Creates a new instance of the symbol table accessor
    pub fn new(elfio: &'a Elfio, section: &'a dyn ElfSectionTrait) -> SymbolSectionAccessor<'a> {
        SymbolSectionAccessor { elfio, section }
    }

    /// Returns number of symbols
    pub fn get_symbols_num(&self) -> ElfXword {
        if self.section.get_entry_size() != 0 {
            return self.section.get_size() / self.section.get_entry_size();
        }

        0
    }

    /// Get a symbol by its index
    pub fn get_symbol(&self, index: ElfXword) -> Option<Symbol> {
        let symbols_num = self.get_symbols_num();
        if symbols_num == 0 || index > symbols_num - 1 {
            return None;
        }

        let offset: usize = (index * self.section.get_entry_size()) as usize;
        let end: usize = offset + self.section.get_entry_size() as usize;
        let symbol_area = &self.section.get_data()[offset..end];

        let converter = self.elfio.get_converter();

        if self.elfio.get_class() == constant::ELFCLASS64 {
            let sym = Elf64Sym {
                st_name:  converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&symbol_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                st_info:  converter.convert(symbol_area[4]),
                st_other: converter.convert(symbol_area[5]),
                st_shndx: converter.convert(u16::from_ne_bytes(
                    <[u8; 2]>::try_from(&symbol_area[6..8]).unwrap_or([0u8, 0u8]),
                )),
                st_value: converter.convert(u64::from_ne_bytes(
                    <[u8; 8]>::try_from(&symbol_area[8..16])
                        .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
                )),
                st_size:  converter.convert(u64::from_ne_bytes(
                    <[u8; 8]>::try_from(&symbol_area[16..24])
                        .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
                )),
            };

            let string_section = self
                .elfio
                .get_section_by_index(self.section.get_link() as ElfHalf);
            let string_accessor = StringSectionAccessor::new(self.elfio, string_section.unwrap());
            let name = string_accessor.get_string(sym.st_name);

            Some(Symbol {
                name,
                value: sym.st_value,
                size: sym.st_size,
                bind: sym.st_info >> 4,
                stype: sym.st_info & 0xF,
                other: sym.st_other,
                shndx: sym.st_shndx,
            })
        } else {
            let sym = Elf32Sym {
                st_name:  converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&symbol_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                st_value: converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&symbol_area[4..8]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                st_size:  converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&symbol_area[8..12]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                st_info:  converter.convert(symbol_area[12]),
                st_other: converter.convert(symbol_area[13]),
                st_shndx: converter.convert(u16::from_ne_bytes(
                    <[u8; 2]>::try_from(&symbol_area[14..16]).unwrap_or([0u8, 0u8]),
                )),
            };

            let string_section = self
                .elfio
                .get_section_by_index(self.section.get_link() as ElfHalf);
            let string_accessor = StringSectionAccessor::new(self.elfio, string_section.unwrap());
            let name = string_accessor.get_string(sym.st_name);

            Some(Symbol {
                name,
                value: sym.st_value as u64,
                size: sym.st_size as u64,
                bind: sym.st_info >> 4,
                stype: sym.st_info & 0xF,
                other: sym.st_other,
                shndx: sym.st_shndx,
            })
        }
    }
}
