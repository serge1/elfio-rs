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

use std::fs::File;
use std::io;
use std::io::BufReader;

use elfio::Elfio;

#[test]
fn read_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    assert_eq!(elf.get_class(), elfio::ELFCLASS32);
    assert_eq!(elf.get_elf_version(), 1);
    assert_eq!(elf.get_encoding(), elfio::ELFDATA2LSB);
    assert_eq!(elf.get_header_size(), 52);
    assert_eq!(elf.get_section_entry_size(), 40);
    assert_eq!(elf.get_segment_entry_size(), 32);
    assert_eq!(elf.get_version(), 1);
    assert_eq!(elf.get_os_abi(), 0);
    assert_eq!(elf.get_abi_version(), 0);
    assert_eq!(elf.get_type(), elfio::ET_EXEC);
    assert_eq!(elf.get_machine(), 3);
    assert_eq!(elf.get_flags(), 0);
    assert_eq!(elf.get_entry(), 0x80482b0);
    assert_eq!(elf.get_sections_num(), 28);
    assert_eq!(elf.get_sections_offset(), 1912);
    assert_eq!(elf.get_segments_num(), 7);
    assert_eq!(elf.get_segments_offset(), 52);
    assert_eq!(elf.get_section_name_str_index(), 25);

    let sections = elf.get_sections();
    assert_eq!(sections.len(), 28);

    let section = sections.get(0).unwrap();
    assert_eq!(section.get_type(), 0);
    assert_eq!(section.get_flags(), 0);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 0);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0);
    assert_eq!(section.get_size(), 0);
    assert_eq!(section.get_offset(), 0);

    let section = sections.get(12).unwrap();
    assert_eq!(section.get_type(), elfio::SHT_PROGBITS);
    assert_eq!(section.get_flags(), elfio::SHF_ALLOC + elfio::SHF_EXECINSTR);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 16);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0x080482b0);
    assert_eq!(section.get_size(), 0x1a8);
    assert_eq!(section.get_offset(), 0x2b0);

    let section = sections.get(19).unwrap();
    assert_eq!(section.get_type(), elfio::SHT_DYNAMIC);
    assert_eq!(section.get_flags(), elfio::SHF_ALLOC + elfio::SHF_WRITE);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 5);
    assert_eq!(section.get_addr_align(), 4);
    assert_eq!(section.get_entry_size(), 8);
    assert_eq!(section.get_address(), 0x080494a0);
    assert_eq!(section.get_size(), 0xc8);
    assert_eq!(section.get_offset(), 0x4a0);

    let segments = elf.get_segments();
    assert_eq!(segments.len(), 7);

    let segment = segments.get(0).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_PHDR);
    assert_eq!(segment.get_offset(), 0x000034);
    assert_eq!(segment.get_virtual_address(), 0x08048034);
    assert_eq!(segment.get_physical_address(), 0x08048034);
    assert_eq!(segment.get_file_size(), 0x000e0);
    assert_eq!(segment.get_memory_size(), 0x000e0);
    assert_eq!(segment.get_flags(), elfio::PF_R + elfio::PF_X);
    assert_eq!(segment.get_align(), 4);

    let segment = segments.get(1).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_INTERP);
    assert_eq!(segment.get_offset(), 0x000114);
    assert_eq!(segment.get_virtual_address(), 0x08048114);
    assert_eq!(segment.get_physical_address(), 0x08048114);
    assert_eq!(segment.get_file_size(), 0x00013);
    assert_eq!(segment.get_memory_size(), 0x00013);
    assert_eq!(segment.get_flags(), elfio::PF_R);
    assert_eq!(segment.get_align(), 1);

    let segment = segments.get(4).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_DYNAMIC);
    assert_eq!(segment.get_offset(), 0x0004a0);
    assert_eq!(segment.get_virtual_address(), 0x080494a0);
    assert_eq!(segment.get_physical_address(), 0x080494a0);
    assert_eq!(segment.get_file_size(), 0x000c8);
    assert_eq!(segment.get_memory_size(), 0x000c8);
    assert_eq!(segment.get_flags(), elfio::PF_R + elfio::PF_W);
    assert_eq!(segment.get_align(), 4);

    Ok(())
}

#[test]
fn read_le_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    assert_eq!(elf.get_class(), elfio::ELFCLASS64);
    assert_eq!(elf.get_elf_version(), 1);
    assert_eq!(elf.get_encoding(), elfio::ELFDATA2LSB);
    assert_eq!(elf.get_header_size(), 64);
    assert_eq!(elf.get_section_entry_size(), 64);
    assert_eq!(elf.get_segment_entry_size(), 56);
    assert_eq!(elf.get_version(), 1);
    assert_eq!(elf.get_os_abi(), 0);
    assert_eq!(elf.get_abi_version(), 0);
    assert_eq!(elf.get_type(), elfio::ET_EXEC);
    assert_eq!(elf.get_machine(), 62);
    assert_eq!(elf.get_flags(), 0);
    assert_eq!(elf.get_entry(), 0x4003c0);
    assert_eq!(elf.get_sections_num(), 29);
    assert_eq!(elf.get_sections_offset(), 2656);
    assert_eq!(elf.get_segments_num(), 8);
    assert_eq!(elf.get_segments_offset(), 64);
    assert_eq!(elf.get_section_name_str_index(), 26);

    let sections = elf.get_sections();

    let section = sections.get(0).unwrap();
    assert_eq!(section.get_type(), 0);
    assert_eq!(section.get_flags(), 0);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 0);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0);
    assert_eq!(section.get_size(), 0);
    assert_eq!(section.get_offset(), 0);

    let section = sections.get(1).unwrap();
    assert_eq!(section.get_type(), elfio::SHT_PROGBITS);
    assert_eq!(section.get_flags(), elfio::SHF_ALLOC);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 1);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0x400200);
    assert_eq!(section.get_size(), 0x1c);
    assert_eq!(section.get_offset(), 0x200);

    let segments = elf.get_segments();
    assert_eq!(segments.len(), 8);

    let segment = segments.get(0).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_PHDR);
    assert_eq!(segment.get_offset(), 0x000040);
    assert_eq!(segment.get_virtual_address(), 0x0000000000400040);
    assert_eq!(segment.get_physical_address(), 0x0000000000400040);
    assert_eq!(segment.get_file_size(), 0x00000000000001c0);
    assert_eq!(segment.get_memory_size(), 0x00000000000001c0);
    assert_eq!(segment.get_flags(), elfio::PF_R + elfio::PF_X);
    assert_eq!(segment.get_align(), 8);

    let segment = segments.get(1).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_INTERP);
    assert_eq!(segment.get_offset(), 0x000200);
    assert_eq!(segment.get_virtual_address(), 0x400200);
    assert_eq!(segment.get_physical_address(), 0x400200);
    assert_eq!(segment.get_file_size(), 0x0001c);
    assert_eq!(segment.get_memory_size(), 0x0001c);
    assert_eq!(segment.get_flags(), elfio::PF_R);
    assert_eq!(segment.get_align(), 1);

    let segment = segments.get(4).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_DYNAMIC);
    assert_eq!(segment.get_offset(), 0x0000000000000698);
    assert_eq!(segment.get_virtual_address(), 0x0000000000600698);
    assert_eq!(segment.get_physical_address(), 0x0000000000600698);
    assert_eq!(segment.get_file_size(), 0x0000000000000190);
    assert_eq!(segment.get_memory_size(), 0x0000000000000190);
    assert_eq!(segment.get_flags(), elfio::PF_R + elfio::PF_W);
    assert_eq!(segment.get_align(), 8);

    Ok(())
}

#[test]
fn write_read_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    elf.set_version(1000);
    elf.set_os_abi(83);
    elf.set_abi_version(84);
    elf.set_type(1003);
    elf.set_machine(1005);
    elf.set_flags(1006);
    elf.set_entry(1007);
    elf.set_sections_num(1008);
    elf.set_sections_offset(1009);
    elf.set_segments_num(10010);
    elf.set_segments_offset(10011);
    elf.set_section_name_str_index(10012);

    assert_eq!(elf.get_version(), 1000);
    assert_eq!(elf.get_os_abi(), 83);
    assert_eq!(elf.get_abi_version(), 84);
    assert_eq!(elf.get_type(), 1003);
    assert_eq!(elf.get_machine(), 1005);
    assert_eq!(elf.get_flags(), 1006);
    assert_eq!(elf.get_entry(), 1007);
    assert_eq!(elf.get_sections_num(), 1008);
    assert_eq!(elf.get_sections_offset(), 1009);
    assert_eq!(elf.get_segments_num(), 10010);
    assert_eq!(elf.get_segments_offset(), 10011);
    assert_eq!(elf.get_section_name_str_index(), 10012);

    Ok(())
}

#[test]
fn read_be_ppc() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    assert_eq!(elf.get_class(), elfio::ELFCLASS32);
    assert_eq!(elf.get_elf_version(), 1);
    assert_eq!(elf.get_encoding(), elfio::ELFDATA2MSB);
    assert_eq!(elf.get_header_size(), 52);
    assert_eq!(elf.get_section_entry_size(), 40);
    assert_eq!(elf.get_segment_entry_size(), 32);
    assert_eq!(elf.get_version(), 1);
    assert_eq!(elf.get_os_abi(), 0);
    assert_eq!(elf.get_abi_version(), 0);
    assert_eq!(elf.get_type(), elfio::ET_EXEC);
    assert_eq!(elf.get_machine(), 20);
    assert_eq!(elf.get_flags(), 0);
    assert_eq!(elf.get_entry(), 0x10000550);
    assert_eq!(elf.get_sections_num(), 31);
    assert_eq!(elf.get_sections_offset(), 3484);
    assert_eq!(elf.get_segments_num(), 8);
    assert_eq!(elf.get_segments_offset(), 52);
    assert_eq!(elf.get_section_name_str_index(), 28);

    Ok(())
}

#[test]
fn read_be_ppc64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    assert_eq!(elf.get_class(), elfio::ELFCLASS64);
    assert_eq!(elf.get_elf_version(), 1);
    assert_eq!(elf.get_encoding(), elfio::ELFDATA2MSB);
    assert_eq!(elf.get_header_size(), 64);
    assert_eq!(elf.get_section_entry_size(), 64);
    assert_eq!(elf.get_segment_entry_size(), 56);
    assert_eq!(elf.get_version(), 1);
    assert_eq!(elf.get_os_abi(), 0);
    assert_eq!(elf.get_abi_version(), 0);
    assert_eq!(elf.get_type(), elfio::ET_DYN);
    assert_eq!(elf.get_machine(), 21);
    assert_eq!(elf.get_flags(), 1);
    assert_eq!(elf.get_entry(), 0x1fa80);
    assert_eq!(elf.get_sections_num(), 29);
    assert_eq!(elf.get_sections_offset(), 67384);
    assert_eq!(elf.get_segments_num(), 8);
    assert_eq!(elf.get_segments_offset(), 64);
    assert_eq!(elf.get_section_name_str_index(), 28);

    let sections = elf.get_sections();
    let section = sections.get(0).unwrap();
    assert_eq!(section.get_type(), 0);
    assert_eq!(section.get_flags(), 0);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 0);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0);
    assert_eq!(section.get_size(), 0);
    assert_eq!(section.get_offset(), 0);

    let section = sections.get(24).unwrap();
    assert_eq!(section.get_type(), elfio::SHT_PROGBITS);
    assert_eq!(section.get_flags(), elfio::SHF_ALLOC + elfio::SHF_WRITE);
    assert_eq!(section.get_info(), 0);
    assert_eq!(section.get_link(), 0);
    assert_eq!(section.get_addr_align(), 8);
    assert_eq!(section.get_entry_size(), 0);
    assert_eq!(section.get_address(), 0x20000);
    assert_eq!(section.get_size(), 0x5a4);
    assert_eq!(section.get_offset(), 0x10000);

    let segments = elf.get_segments();
    assert_eq!(segments.len(), 8);

    let segment = segments.get(0).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_PHDR);
    assert_eq!(segment.get_offset(), 0x000040);
    assert_eq!(segment.get_virtual_address(), 0x0000000000000040);
    assert_eq!(segment.get_physical_address(), 0x0000000000000040);
    assert_eq!(segment.get_file_size(), 0x00000000000001c0);
    assert_eq!(segment.get_memory_size(), 0x00000000000001c0);
    assert_eq!(segment.get_flags(), elfio::PF_R);
    assert_eq!(segment.get_align(), 8);

    let segment = segments.get(1).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_INTERP);
    assert_eq!(segment.get_offset(), 0x000200);
    assert_eq!(segment.get_virtual_address(), 0x200);
    assert_eq!(segment.get_physical_address(), 0x200);
    assert_eq!(segment.get_file_size(), 0x00011);
    assert_eq!(segment.get_memory_size(), 0x00011);
    assert_eq!(segment.get_flags(), elfio::PF_R);
    assert_eq!(segment.get_align(), 1);

    let segment = segments.get(4).unwrap();
    assert_eq!(segment.get_type(), elfio::PT_DYNAMIC);
    assert_eq!(segment.get_offset(), 0x000000000000f880);
    assert_eq!(segment.get_virtual_address(), 0x000000000001f880);
    assert_eq!(segment.get_physical_address(), 0x000000000001f880);
    assert_eq!(segment.get_file_size(), 0x0000000000000200);
    assert_eq!(segment.get_memory_size(), 0x0000000000000200);
    assert_eq!(segment.get_flags(), elfio::PF_R + elfio::PF_W);
    assert_eq!(segment.get_align(), 8);

    Ok(())
}
