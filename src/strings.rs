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

trait StringSectionAccessorTrait {
    // Returns a string from string table by its index
    fn get_string1(&self, index: ElfWord) -> String;
}

/// A section data accessor dedicated to string tables
///
/// For example:
/// ```
/// use std::fs::File;
/// use std::io;
/// use std::io::{BufReader, Error};
/// use elfio::{Elfio, StringSectionAccessor}; // Uncomment in the real code
///
/// fn main() -> io::Result<()> {
///     let elf_file = File::open("tests/files/hello_64")?;
///     let mut file_reader = BufReader::new(elf_file);
///
///     let mut elf = elfio::Elfio::new();
///
///     elf.load(&mut file_reader)?;
///
///     let section = elf.get_section_by_name(&".strtab".to_string());
///
///     match section {
///         Some(s) => {
///             let strtab = StringSectionAccessor::new(s);
///             println!("{}", strtab.get_string(1));
///         }
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     }
///
///     Ok(())
/// }
/// ```

pub struct StringSectionAccessor<'a> {
    section: &'a Box<dyn ElfSectionTrait>,
}

impl<'a> StringSectionAccessor<'a> {
    /// Creates a new instance of the string table accessor
    pub fn new(section: &'a Box<dyn ElfSectionTrait>) -> StringSectionAccessor<'a> {
        StringSectionAccessor { section: section }
    }

    /// Returns a string from string table by its index
    pub fn get_string(&self, index: ElfWord) -> String {
        let strdata = self.section.get_data();
        let ret = Elfio::str_from_u8_nul_utf8_unchecked(&strdata[index as usize..]).to_string();

        return ret;
    }
}

impl<'a> StringSectionAccessorTrait for StringSectionAccessor<'a> {
    fn get_string1(&self, index: ElfWord) -> String {
        let strdata = self.section.get_data();
        let ret = Elfio::str_from_u8_nul_utf8_unchecked(&strdata[index as usize..]).to_string();

        return ret;
    }
}