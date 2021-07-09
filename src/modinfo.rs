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

use super::*;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

/// A section data accessor intended to modinfo tables. The accessor is useful
/// for kernel modules data
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
///     let elf_file = File::open("tests/files/i2c-gpio.ko")?;
///     let mut reader = BufReader::new(elf_file);
///
///     let mut elf = Elfio::new();
///
///     elf.load(&mut reader)?;
///
///     let section = match elf.get_section_by_name(&".modinfo") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
/// 
///     let modinfo = elfio::ModInfoSectionAccessor::new(&elf, section);
/// 
///     assert_eq!(modinfo.get_entries_num(), 10);
///     assert_eq!(modinfo.get(&"description".to_string()).unwrap(), "Platform-independent bitbanging I2C driver");
///
///     Ok(())
/// }
/// ```
pub struct ModInfoSectionAccessor<'a> {
    _elfio:   &'a Elfio,
    _section: &'a dyn ElfSectionTrait,
    content: HashMap<String, String>,
}

impl<'a> ModInfoSectionAccessor<'a> {
    /// Creates a new instance of the relocation table accessor
    pub fn new(elfio: &'a Elfio, section: &'a dyn ElfSectionTrait) -> ModInfoSectionAccessor<'a> {
        let mut mi = ModInfoSectionAccessor {
            _elfio: elfio,
            _section: section,
            content: HashMap::new(),
        };

        let data = section.get_data();
        let datalen = data.len();

        if datalen > 0 {
            let mut i: usize = 0;
            while i < datalen {
                while i < datalen && data[i] != 0 {
                    let mut str = Self::str_from_u8_nul_utf8_unchecked(&data[i..]).to_string();
                    i += str.len() + 1;
                    let pos = str.find('=').unwrap();
                    let value = str.split_off(pos + 1);
                    str.truncate(pos);
                    mi.content.insert(str, value);
                }
            }
        }

        mi
    }

    // --------------------------------------------------------------------------
    /// Find and return null terminated byte sequence as a string slice
    fn str_from_u8_nul_utf8_unchecked(utf8_src: &[u8]) -> &str {
        let nul_range_end = utf8_src
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(utf8_src.len()); // default to length if no `\0` present
        unsafe { ::std::str::from_utf8_unchecked(&utf8_src[0..nul_range_end]) }
    }

    /// Returns the number of modinfo entries
    pub fn get_entries_num(&self) -> ElfXword {
        self.content.len() as ElfXword
    }

    /// Get iterator over all modinfo entries
    pub fn get_iter(&self) -> Iter<String, String> {
        return self.content.iter();
    }

    /// Retrieve a value by its key
    pub fn get(&self, field: &String) -> Option<&String> {
        self.content.get(field)
    }
}
