/*
Copyright (C) 2001-present by Serge Lamikhov-Center

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
use num::{cast::AsPrimitive, Zero};

// --------------------------------------------------------------------------
pub trait ElfSectionAccessTrait {
    ELFIO_GET_SET_ACCESS_DECL!(String, name);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, type);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, flags);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, info);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, link);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, addr_align);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, entry_size);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, address);
    ELFIO_GET_SET_ACCESS_DECL!(ElfXword, size);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, name_string_offset);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, offset);

    fn set_converter(&mut self, converter: &Converter);
}

// --------------------------------------------------------------------------
pub trait ElfSectionTrait: ElfSectionAccessTrait + Load {}

// --------------------------------------------------------------------------
// ELF file header
#[repr(C)]
#[derive(Debug)]
pub struct ElfSection<Addr, Offset> {
    sh_name: String,
    sh_type: ElfWord,
    sh_flags: ElfWord,
    sh_addr: Addr,
    sh_offset: Offset,
    sh_size: ElfWord,
    sh_link: ElfWord,
    sh_info: ElfWord,
    sh_addralign: ElfWord,
    sh_entsize: ElfWord,

    converter: Converter,
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfSection<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    pub fn new() -> ElfSection<Addr, Offset> {
        ElfSection::<Addr, Offset> {
            converter: Converter { is_needed: false },

            sh_name: "".to_string(),
            sh_type: 0,
            sh_flags: 0,
            sh_addr: Addr::zero(),
            sh_offset: Offset::zero(),
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfSectionTrait for ElfSection<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
}

// --------------------------------------------------------------------------
impl<Addr, Offset> ElfSectionAccessTrait for ElfSection<Addr, Offset>
where
    u32: AsPrimitive<Addr> + AsPrimitive<Offset>,
    u64: AsPrimitive<Addr> + AsPrimitive<Offset>,
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    ELFIO_GET_SET_ACCESS!(ElfWord, type, sh_type);
    ELFIO_GET_SET_ACCESS!(ElfXword, flags, sh_flags);
    ELFIO_GET_SET_ACCESS!(ElfWord, info, sh_info);
    ELFIO_GET_SET_ACCESS!(ElfWord, link, sh_link);
    ELFIO_GET_SET_ACCESS!(ElfXword, addr_align, sh_addralign);
    ELFIO_GET_SET_ACCESS!(ElfXword, entry_size, sh_entsize);
    ELFIO_GET_SET_ACCESS!(Elf64Addr, address, sh_addr);
    ELFIO_GET_SET_ACCESS!(ElfXword, size, sh_size);
    ELFIO_GET_SET_ACCESS!(ElfWord, name_string_offset, sh_addr);
    ELFIO_GET_SET_ACCESS!(Elf64Off, offset, sh_offset);

    fn get_name(&self) -> std::string::String {
        todo!()
    }
    fn set_name(&mut self, _: std::string::String) {
        todo!()
    }

    fn set_converter(&mut self, converter: &Converter) {
        self.converter = *converter;
    }
}

// --------------------------------------------------------------------------
impl<Addr, Offset> Load for ElfSection<Addr, Offset>
where
    Addr: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Offset: Zero + Load + AsPrimitive<u32> + AsPrimitive<u64>,
    Converter: Convert<Addr> + Convert<Offset>,
{
    fn load(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
        self.sh_type.load(reader)?;
        self.sh_flags.load(reader)?;
        self.sh_info.load(reader)?;
        self.sh_link.load(reader)?;
        self.sh_addralign.load(reader)?;
        self.sh_entsize.load(reader)?;
        self.sh_addr.load(reader)?;
        self.sh_size.load(reader)?;
        self.sh_addr.load(reader)?;
        self.sh_offset.load(reader)?;

        self.sh_type = self.converter.convert(self.sh_type);
        self.sh_flags = self.converter.convert(self.sh_flags);
        self.sh_info = self.converter.convert(self.sh_info);
        self.sh_link = self.converter.convert(self.sh_link);
        self.sh_addralign = self.converter.convert(self.sh_addralign);
        self.sh_entsize = self.converter.convert(self.sh_entsize);
        self.sh_addr = self.converter.convert(self.sh_addr);
        self.sh_size = self.converter.convert(self.sh_size);
        self.sh_addr = self.converter.convert(self.sh_addr);
        self.sh_offset = self.converter.convert(self.sh_offset);

        Ok(())
    }
}
