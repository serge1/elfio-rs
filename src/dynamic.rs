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

/// A dynamic table element
///
/// See documentation for [DynamicSectionAccessor] for usage example
#[derive(Debug, Default)]
pub struct Dynamic {
    /// Identifies the type of dynamic table entry
    pub tag:   ElfSxword,
    /// It is used to represent integer values or program virtual addresses
    pub value: ElfXword,
}

#[repr(C)]
#[derive(Default)]
struct Elf32Dyn {
    d_tag:   ElfSword,
    d_value: ElfWord,
}

#[repr(C)]
#[derive(Default)]
struct Elf64Dyn {
    d_tag:   ElfSxword,
    d_value: ElfXword,
}

/// A section data accessor intended to dynamic tables
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
///     let section = match elf.get_section_by_name(&".dynamic") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
///
///     let dyns = elfio::DynamicSectionAccessor::new(&elf, section);
///
///     assert_eq!(dyns.get_entries_num(), 20);
///
///     // Dynamic section at offset 0x4a0 contains 20 entries:
///     //   Tag        Type                         Name/Value
///     //  0x00000001 (NEEDED)                     Shared library: [libc.so.6]
///     //  0x0000000c (INIT)                       0x804824c
///     //  0x0000000d (FINI)                       0x8048458
///     //  0x6ffffef5 (GNU_HASH)                   0x8048148
///     let dynamic = dyns.get_entry(3).unwrap();
///     assert_eq!(dynamic.tag, 0x6ffffef5);
///     assert_eq!(dynamic.value, 0x8048148);
///
///     Ok(())
/// }
/// ```
pub struct DynamicSectionAccessor<'a> {
    elfio:   &'a Elfio,
    section: &'a dyn ElfSectionTrait,
}

impl<'a> DynamicSectionAccessor<'a> {
    /// Creates a new instance of the relocation table accessor
    pub fn new(elfio: &'a Elfio, section: &'a dyn ElfSectionTrait) -> DynamicSectionAccessor<'a> {
        DynamicSectionAccessor { elfio, section }
    }

    /// Returns number of symbols
    pub fn get_entries_num(&self) -> ElfXword {
        let max_entries = self.get_entries_num_internal();

        for i in 0..max_entries {
            let entry = self.get_entry(i).unwrap();
            if entry.tag == constant::DT_NULL {
                return i + 1;
            }
        }

        0
    }

    // Returns number of symbols according to the section size
    fn get_entries_num_internal(&self) -> ElfXword {
        if self.section.get_entry_size() != 0 {
            return self.section.get_size() / self.section.get_entry_size();
        }

        0
    }

    /// Get a relocation entry by its index
    pub fn get_entry(&self, index: ElfXword) -> Option<Dynamic> {
        let entries_num = self.get_entries_num_internal();
        if entries_num == 0 || index > entries_num - 1 {
            return None;
        }

        let offset: usize = (index * self.section.get_entry_size()) as usize;
        let end: usize = offset + self.section.get_entry_size() as usize;
        let entry_area = &self.section.get_data()[offset..end];

        let converter = self.elfio.get_converter();

        if self.elfio.get_class() == constant::ELFCLASS64 {
            let mut entry: Elf64Dyn = Default::default();
            entry.d_tag = converter.convert(i64::from_ne_bytes(
                <[u8; 8]>::try_from(&entry_area[0..8])
                    .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            ));
            entry.d_value = converter.convert(u64::from_ne_bytes(
                <[u8; 8]>::try_from(&entry_area[8..16])
                    .unwrap_or([0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]),
            ));

            Some(Dynamic {
                tag:   entry.d_tag,
                value: entry.d_value,
            })
        } else {
            let mut entry: Elf32Dyn = Default::default();
            entry.d_tag = converter.convert(i32::from_ne_bytes(
                <[u8; 4]>::try_from(&entry_area[0..4]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));
            entry.d_value = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&entry_area[4..8]).unwrap_or([0u8, 0u8, 0u8, 0u8]),
            ));

            Some(Dynamic {
                tag:   entry.d_tag as ElfSxword,
                value: entry.d_value as ElfXword,
            })
        }
    }
}
