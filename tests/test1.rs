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
fn_read_le_32() -> io::Result<()> {
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

    Ok(())
}

#[test]
fn_read_le_64() -> io::Result<()> {
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

    Ok(())
}

#[test]
fn_write_read_le_32() -> io::Result<()> {
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
fn_read_be_ppc() -> io::Result<()> {
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
fn_read_be_ppc64() -> io::Result<()> {
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

    Ok(())
}
