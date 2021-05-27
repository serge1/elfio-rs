use std::fs::File;
use std::io;

use elfio::elfio::Elfio;

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
