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

/// A struct represents a single symbol from symbol table section
#[repr(C)]
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

#[derive(Default)]
struct Elf32Sym {
    st_name:  ElfWord,
    st_value: Elf32Addr,
    st_size:  ElfWord,
    st_info:  u8,
    st_other: u8,
    st_shndx: ElfHalf,
}

#[derive(Default)]
struct Elf64Sym {
    st_name:  ElfWord,
    st_info:  u8,
    st_other: u8,
    st_shndx: ElfHalf,
    st_value: Elf64Addr,
    st_size:  ElfXword,
}

/// A section data accessor intended to symbol tables
pub struct SymbolSectionAccessor<'a> {
    elfio:   &'a Elfio,
    section: &'a dyn ElfSectionTrait,
}

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
        if index > self.get_symbols_num() - 1 {
            return None;
        }
        let offset: usize = (index * self.section.get_entry_size()) as usize;
        let end: usize = offset + self.section.get_entry_size() as usize;
        let symbol_area = &self.section.get_data()[offset..end];

        let converter = self.elfio.get_converter();

        if self.elfio.get_class() == ELFCLASS64 {
            let mut sym: Elf64Sym = Default::default();
            sym.st_name = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&symbol_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));
            sym.st_info = converter.convert(symbol_area[4]);
            sym.st_other = converter.convert(symbol_area[5]);
            sym.st_shndx = converter.convert(u16::from_ne_bytes(
                <[u8; 2]>::try_from(&symbol_area[6..8]).unwrap_or([0u8, 0u8]),
            ));
            sym.st_value = converter.convert(u64::from_ne_bytes(
                <[u8; 8]>::try_from(&symbol_area[8..16])
                    .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            ));
            sym.st_size = converter.convert(u64::from_ne_bytes(
                <[u8; 8]>::try_from(&symbol_area[16..24])
                    .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            ));

            let string_section = self
                .elfio
                .get_section_by_index(self.section.get_link() as ElfHalf);
            let string_accessor = StringSectionAccessor::new(string_section.unwrap());
            let name = string_accessor.get_string(sym.st_name);

            Some(Symbol {
                name:  name,
                value: sym.st_value,
                size:  sym.st_size,
                bind:  sym.st_info >> 4,
                stype: sym.st_info & 0xF,
                other: sym.st_other,
                shndx: sym.st_shndx,
            })
        } else {
            let mut sym: Elf32Sym = Default::default();
            sym.st_name = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&symbol_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));
            sym.st_value = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&symbol_area[4..8]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));
            sym.st_size = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&symbol_area[8..12]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));
            sym.st_info = converter.convert(symbol_area[12]);
            sym.st_other = converter.convert(symbol_area[13]);
            sym.st_shndx = converter.convert(u16::from_ne_bytes(
                <[u8; 2]>::try_from(&symbol_area[14..16]).unwrap_or([0u8, 0u8]),
            ));

            let string_section = self
                .elfio
                .get_section_by_index(self.section.get_link() as ElfHalf);
            let string_accessor = StringSectionAccessor::new(string_section.unwrap());
            let name = string_accessor.get_string(sym.st_name);

            Some(Symbol {
                name:  name,
                value: sym.st_value as u64,
                size:  sym.st_size as u64,
                bind:  sym.st_info >> 4,
                stype: sym.st_info & 0xF,
                other: sym.st_other,
                shndx: sym.st_shndx,
            })
        }
    }
}
