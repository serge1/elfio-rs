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

/// A note entry. The note is used by compilers and other tools
/// to mark an object file with special information that has special
/// meaning to a particular tool set.
pub struct Note {
    /// A number that determines, along with the originator’s name,
    /// the interpretation of the note contents
    pub ntype:       ElfWord,
    /// A name identifying the entry’s owner or originator
    pub name:        String,
    /// The contents of the note. The format and interpretation of
    /// the note contents are determined solely by the name and type fields
    pub description: Vec<u8>,
}

/// A section data accessor intended to note sections
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
///     let section = match elf.get_section_by_name(&".note.ABI-tag") {
///         Some(s) => s,
///         None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
///     };
///
///     let notes = elfio::NoteSectionAccessor::new(&elf, section);
///
///     assert_eq!(notes.get_notes_num(), 1);
///
///     let note = notes.get_note(0).unwrap();
///     assert_eq!(note.ntype, 1);
///     assert_eq!(note.name, "GNU");
///     assert_eq!(
///         note.description,
///         vec![0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8]
///     );
///
///     Ok(())
/// }
/// ```
pub struct NoteSectionAccessor<'a> {
    elfio:                &'a Elfio,
    section:              &'a dyn ElfSectionTrait,
    note_start_positions: Vec<usize>,
}

impl<'a> NoteSectionAccessor<'a> {
    /// Creates a new instance of the symbol table accessor
    pub fn new(elfio: &'a Elfio, section: &'a dyn ElfSectionTrait) -> NoteSectionAccessor<'a> {
        let mut note_accessor = NoteSectionAccessor {
            elfio,
            section,
            note_start_positions: Vec::<usize>::new(),
        };

        let data = section.get_data();
        let size = data.len();
        if data.len() == 0 {
            return note_accessor;
        }

        let align = std::mem::size_of::<ElfWord>();
        let mut current: usize = 0;
        while (current + 3 * align) <= size {
            note_accessor.note_start_positions.push(current);

            let converter = elfio.get_converter();
            let namesz = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&data[current..current + 4]).unwrap_or([0x8, 0u8, 0u8, 0u8]),
            ));
            let descsz = converter.convert(u32::from_ne_bytes(
                <[u8; 4]>::try_from(&data[current + 4..current + 8])
                    .unwrap_or([0x8, 0u8, 0u8, 0u8]),
            ));

            current += 3 * align
                + ((namesz as usize + align - 1) / align) * align
                + ((descsz as usize + align - 1) / align) * align;
        }

        note_accessor
    }

    /// Returns number of notes in the section
    pub fn get_notes_num(&self) -> ElfWord {
        self.note_start_positions.len() as ElfWord
    }

    /// Returns a note by its ordinal number
    pub fn get_note(&self, index: ElfWord) -> Option<Note> {
        let index = index as usize;
        let data = self.section.get_data();
        if index >= self.note_start_positions.len() {
            return None;
        }

        let area = &data[self.note_start_positions[index]..];
        let align = std::mem::size_of::<ElfWord>();
        let converter = self.elfio.get_converter();

        let name_size = converter.convert(u32::from_ne_bytes(
            <[u8; 4]>::try_from(&area[0..4]).unwrap_or([0x8, 0u8, 0u8, 0u8]),
        ));
        let desc_size = converter.convert(u32::from_ne_bytes(
            <[u8; 4]>::try_from(&area[4..8]).unwrap_or([0x8, 0u8, 0u8, 0u8]),
        ));
        let ntype = converter.convert(u32::from_ne_bytes(
            <[u8; 4]>::try_from(&area[8..12]).unwrap_or([0x8, 0u8, 0u8, 0u8]),
        ));

        let max_name_size = data.len() - self.note_start_positions[index];
        if name_size < 1
            || name_size > max_name_size as u32
            || name_size + desc_size > max_name_size as u32
        {
            return None;
        }

        Some(Note {
            ntype,
            name: unsafe {
                ::std::str::from_utf8_unchecked(&area[12..12 + name_size as usize - 1]).to_string()
            },
            description: {
                let desc_pos = 12 + ((name_size as usize + align - 1) / align) * align;
                area[desc_pos..desc_pos + desc_size as usize]
                    .iter()
                    .cloned()
                    .collect()
            },
        })
    }
}
