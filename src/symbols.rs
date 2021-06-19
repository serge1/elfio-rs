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

/// A section data accessor intended to symbol tables
pub struct SymbolSectionAccessor<'a> {
    _elfio: &'a Elfio,
    section: &'a Box<dyn ElfSectionTrait>,
}

impl<'a> SymbolSectionAccessor<'a> {
    /// Creates a new instance of the symbol table accessor
    pub fn new(elfio: &'a Elfio, section: &'a Box<dyn ElfSectionTrait>) -> SymbolSectionAccessor<'a> {
        SymbolSectionAccessor { _elfio:elfio, section: section }
    }

    /// Returns number of symbols 
    pub fn get_symbols_num(&self) -> ElfXword {
        if self.section.get_entry_size() != 0 {
            return self.section.get_size() / self.section.get_entry_size()
        }

        return 0;
    }
}
