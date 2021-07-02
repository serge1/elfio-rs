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

use super::utils::*;
use super::*;
use num_traits::{AsPrimitive, Zero};

// --------------------------------------------------------------------------
/// Read/Write access to segment properties
pub trait ElfSegmentAccessTrait {
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, type);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, flags);
    ELFIO_GET_ACCESS_DECL!(Elf64Off, offset);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, virtual_address);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, physical_address);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, file_size);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, memory_size);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, align);
}

// --------------------------------------------------------------------------
pub trait ElfSegmentTrait: ElfSegmentAccessTrait + Load {}

// --------------------------------------------------------------------------
// ELF file header
#[repr(C)]
pub struct ElfSegment<Addr, Offset, Word> {
    p_type:    ElfWord,
    p_flags:   ElfWord,
    p_offset:  Offset,
    p_vaddr:   Addr,
    p_paddr:   Addr,
    p_filesz:  Word,
    p_memsz:   Word,
    p_align:   Word,
    converter: Converter,
    class:     u8,
}

// --------------------------------------------------------------------------
impl<Addr, Offset, Word> ElfSegment<Addr, Offset, Word>
where
    Addr: Zero + Load + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u64>,
    Word: Zero + Load + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset> + Convert<Word>,
{
    pub fn new(conv: &Converter, class: u8) -> ElfSegment<Addr, Offset, Word> {
        Self {
            converter: *conv,
            class,
            p_type: 0,
            p_flags: 0,
            p_offset: Offset::zero(),
            p_vaddr: Addr::zero(),
            p_paddr: Addr::zero(),
            p_filesz: Word::zero(),
            p_memsz: Word::zero(),
            p_align: Word::zero(),
        }
    }
}

// --------------------------------------------------------------------------
impl<Addr, Offset, Word> ElfSegmentTrait for ElfSegment<Addr, Offset, Word>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset> + AsPrimitive<Word>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset> + AsPrimitive<Word>,
    Addr: Zero + Load + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u64>,
    Word: Zero + Load + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset> + Convert<Word>,
{
}

// --------------------------------------------------------------------------
impl<Addr, Offset, Word> ElfSegmentAccessTrait for ElfSegment<Addr, Offset, Word>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset> + AsPrimitive<Word>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset> + AsPrimitive<Word>,
    Addr: Zero + Load + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u64>,
    Word: Zero + Load + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset> + Convert<Word>,
{
    ELFIO_GET_SET_ACCESS!(ElfWord, type, p_type);
    ELFIO_GET_SET_ACCESS!(ElfWord, flags, p_flags);
    ELFIO_GET_ACCESS!(Elf64Off, offset, p_offset);
    ELFIO_GET_SET_ACCESS!(Elf64Addr, virtual_address, p_vaddr);
    ELFIO_GET_SET_ACCESS!(Elf64Addr, physical_address, p_paddr);
    ELFIO_GET_SET_ACCESS!(ElfXword, file_size, p_filesz);
    ELFIO_GET_SET_ACCESS!(ElfXword, memory_size, p_memsz);
    ELFIO_GET_SET_ACCESS!(ElfXword, align, p_align);
}

// --------------------------------------------------------------------------
impl<Addr, Offset, Word> Load for ElfSegment<Addr, Offset, Word>
where
    Addr: Zero + Load + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u64>,
    Word: Zero + Load + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset> + Convert<Word>,
{
    fn load(&mut self, reader: &mut (dyn ElfioReadSeek)) -> io::Result<()> {
        self.p_type.load(reader)?;
        if self.class == ELFCLASS64 {
            self.p_flags.load(reader)?;
            self.p_offset.load(reader)?;
            self.p_vaddr.load(reader)?;
            self.p_paddr.load(reader)?;
            self.p_filesz.load(reader)?;
            self.p_memsz.load(reader)?;
        } else {
            self.p_offset.load(reader)?;
            self.p_vaddr.load(reader)?;
            self.p_paddr.load(reader)?;
            self.p_filesz.load(reader)?;
            self.p_memsz.load(reader)?;
            self.p_flags.load(reader)?;
        }
        self.p_align.load(reader)?;

        Ok(())
    }
}
