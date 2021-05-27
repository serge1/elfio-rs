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

use elfio::Elfio;

#[test]
fn header_read_le_32() -> io::Result<()> {
    let mut elf_file = File::open("tests/files/hello_32")?;

    let mut elfio = Elfio::new();

    elfio.load(&mut elf_file)?;

    assert_eq!(elfio.get_class(), 1);
    assert_eq!(elfio.get_elf_version(), 1);
    assert_eq!(elfio.get_encoding(), 1);
    assert_eq!(elfio.get_header_size(), 52);
    assert_eq!(elfio.get_section_entry_size(), 40);
    assert_eq!(elfio.get_segment_entry_size(), 32);
    assert_eq!(elfio.get_version(), 1);
    assert_eq!(elfio.get_os_abi(), 0);
    assert_eq!(elfio.get_abi_version(), 0);
    assert_eq!(elfio.get_type(), 2);
    assert_eq!(elfio.get_machine(), 3);
    assert_eq!(elfio.get_flags(), 0);
    assert_eq!(elfio.get_entry(), 0x80482b0);
    assert_eq!(elfio.get_sections_num(), 28);
    assert_eq!(elfio.get_sections_offset(), 1912);
    assert_eq!(elfio.get_segments_num(), 7);
    assert_eq!(elfio.get_segments_offset(), 52);
    assert_eq!(elfio.get_section_name_str_index(), 25);

    Ok(())
}

#[test]
fn header_read_le_64() -> io::Result<()> {
    let mut elf_file = File::open("tests/files/hello_64")?;

    let mut elfio = Elfio::new();

    elfio.load(&mut elf_file)?;

    assert_eq!(elfio.get_class(), 2);
    assert_eq!(elfio.get_elf_version(), 1);
    assert_eq!(elfio.get_encoding(), 1);
    assert_eq!(elfio.get_header_size(), 64);
    assert_eq!(elfio.get_section_entry_size(), 64);
    assert_eq!(elfio.get_segment_entry_size(), 56);
    assert_eq!(elfio.get_version(), 1);
    assert_eq!(elfio.get_os_abi(), 0);
    assert_eq!(elfio.get_abi_version(), 0);
    assert_eq!(elfio.get_type(), 2);
    assert_eq!(elfio.get_machine(), 62);
    assert_eq!(elfio.get_flags(), 0);
    assert_eq!(elfio.get_entry(), 0x4003c0);
    assert_eq!(elfio.get_sections_num(), 29);
    assert_eq!(elfio.get_sections_offset(), 2656);
    assert_eq!(elfio.get_segments_num(), 8);
    assert_eq!(elfio.get_segments_offset(), 64);
    assert_eq!(elfio.get_section_name_str_index(), 26);

    Ok(())
}
