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

/// A relocation entry. The relocation contains information that describes
/// how to modify section contents, thus allowing executable and shared object
/// files to hold the right information for a process's program image.
///
/// See documentation for [RelocationSectionAccessor] for usage example
#[derive(Debug, Default)]
pub struct Relocation {
    /// A number that determines, along with the originator’s name,
    /// the interpretation of the note contents
    pub offset: Elf64Addr,
    /// The symbol whose value should be used in the relocation
    pub symbol: ElfWord,
    /// The relocation type. Relocation types are processor specific
    pub rtype:  ElfWord,
    /// Specifies a constant addend used to compute the value to be
    /// stored in the relocated field. The field is valid only for relocation
    /// sections having SHT_RELA type
    pub addend: Option<ElfSxword>,
}

#[repr(C)]
#[derive(Default)]
struct Elf32Rel {
    r_offset: Elf32Addr,
    r_info:   ElfWord,
}

#[repr(C)]
#[derive(Default)]
struct Elf32Rela {
    r_offset: Elf32Addr,
    r_info:   ElfWord,
    r_addend: ElfSword,
}

#[repr(C)]
#[derive(Default)]
struct Elf64Rel {
    r_offset: Elf64Addr,
    r_info:   ElfXword,
}

#[repr(C)]
#[derive(Default)]
struct Elf64Rela {
    r_offset: Elf64Addr,
    r_info:   ElfXword,
    r_addend: ElfSxword,
}

/// A section data accessor intended to relocation tables
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
///     let elf_file = File::open("tests/files/hello_64")?;
///     let mut reader = BufReader::new(elf_file);
///
///     let mut elf = Elfio::new();
///
///     elf.load(&mut reader)?;
///
///     let section = match elf.get_section_by_name(&".rela.plt") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
///
///     let relocs = elfio::RelocationSectionAccessor::new(&elf, section);
///
///     assert_eq!(relocs.get_entries_num(), 2);
///
///     // 000000600850  000300000007 R_X86_64_JUMP_SLO 0000000000000000 __libc_start_main@GLIBC_2.2.5 + 0
///     let rel = relocs.get_entry(1).unwrap();
///     assert_eq!(rel.offset, 0x000000600850);
///     assert_eq!(rel.symbol, 3);
///     assert_eq!(rel.rtype, 7);
///     assert_eq!(rel.addend, Some(0));
///
///     Ok(())
/// }
/// ```
pub struct RelocationSectionAccessor<'a> {
    elfio:   &'a Elfio,
    section: &'a dyn ElfSectionTrait,
}

impl<'a> RelocationSectionAccessor<'a> {
    /// Creates a new instance of the relocation table accessor
    pub fn new(
        elfio: &'a Elfio,
        section: &'a dyn ElfSectionTrait,
    ) -> RelocationSectionAccessor<'a> {
        RelocationSectionAccessor { elfio, section }
    }

    /// Returns number of symbols
    pub fn get_entries_num(&self) -> ElfXword {
        if self.section.get_entry_size() != 0 {
            return self.section.get_size() / self.section.get_entry_size();
        }

        0
    }

    /// Get a relocation entry by its index
    pub fn get_entry(&self, index: ElfXword) -> Option<Relocation> {
        let entries_num = self.get_entries_num();
        if entries_num == 0 || index > entries_num - 1 {
            return None;
        }

        let offset: usize = (index * self.section.get_entry_size()) as usize;
        let end: usize = offset + self.section.get_entry_size() as usize;
        let entry_area = &self.section.get_data()[offset..end];

        let converter = self.elfio.get_converter();

        if self.elfio.get_class() == constant::ELFCLASS64 {
            let mut entry = Elf64Rela {
                r_offset: converter.convert(u64::from_ne_bytes(
                    <[u8; 8]>::try_from(&entry_area[0..8])
                        .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
                )),
                r_info: converter.convert(u64::from_ne_bytes(
                    <[u8; 8]>::try_from(&entry_area[8..16])
                        .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
                )),
                ..Default::default()
            };
            if self.section.get_type() == constant::SHT_RELA {
                entry.r_addend = converter.convert(i64::from_ne_bytes(
                    <[u8; 8]>::try_from(&entry_area[16..24])
                        .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
                ));
            }

            Some(Relocation {
                offset: entry.r_offset as Elf64Addr,
                symbol: (entry.r_info >> 32) as ElfWord,
                rtype:  (entry.r_info & 0xFFFFFFFFu64) as ElfWord,
                addend: if self.section.get_type() == constant::SHT_RELA {
                    Some(entry.r_addend as ElfSxword)
                } else {
                    None
                },
            })
        } else {
            let mut entry = Elf32Rela {
                r_offset: converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&entry_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                r_info: converter.convert(u32::from_ne_bytes(
                    <[u8; 4]>::try_from(&entry_area[4..8]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                )),
                ..Default::default()
            };
            if self.section.get_type() == constant::SHT_RELA {
                entry.r_addend = converter.convert(i32::from_ne_bytes(
                    <[u8; 4]>::try_from(&entry_area[8..12]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
                ));
            }

            Some(Relocation {
                offset: entry.r_offset as Elf64Addr,
                symbol: (entry.r_info >> 8) as ElfWord,
                rtype:  (entry.r_info & 0xFFu32) as ElfWord,
                addend: if self.section.get_type() == constant::SHT_RELA {
                    Some(entry.r_addend as ElfSxword)
                } else {
                    None
                },
            })
        }
    }
}
