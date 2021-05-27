extern crate num;

use num::cast::AsPrimitive;
use num::{Num, NumCast, Zero};
use paste::paste;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::slice;

use super::types::*;

// Identification index
pub const EI_MAG0: usize = 0;
pub const EI_MAG1: usize = 1;
pub const EI_MAG2: usize = 2;
pub const EI_MAG3: usize = 3;
pub const EI_CLASS: usize = 4;
pub const EI_DATA: usize = 5;
pub const EI_VERSION: usize = 6;
pub const EI_OSABI: usize = 7;
pub const EI_ABIVERSION: usize = 8;
// pub const EI_PAD: usize = 9;
pub const EI_NIDENT: usize = 16;

// Magic number
pub const ELFMAG0: u8 = 0x7F;
pub const ELFMAG1: u8 = 'E' as u8;
pub const ELFMAG2: u8 = 'L' as u8;
pub const ELFMAG3: u8 = 'F' as u8;

// File class
//pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;

// Encoding
//pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;

macro_rules! ELFIO_GET_ACCESS_DECL {
    ($type: ident, $name: ident) => {
        paste! {
            fn [<get_ $name>](&self) -> $type;
        }
    };
}

macro_rules! ELFIO_GET_SET_ACCESS_DECL {
    ($type: ident, $name: ident) => {
        paste! {
            fn [<get_ $name>](&self) -> $type;
            // fn [<set_ $name>](&mut self, value: $type) -> ();
        }
    };
}

macro_rules! ELFIO_GET_ACCESS {
    ($type: ident, $name: ident, $field: expr) => {
        paste! {
            fn [<get_ $name>](&self) -> $type {
                paste! [self. $field].as_()
            }
        }
    };
}

macro_rules! ELFIO_GET_SET_ACCESS {
    ($type: ident, $name: ident, $field: expr) => {
        paste! {
            fn [<get_ $name>](&self) -> $type {
                paste! [self. $field].as_()
            }
            // fn [<set_ $name>](&mut self, value: $type) -> () {
            //     paste! [self. $field] = (value).as_();
            // }
        }
    };
}

pub trait ElfHeaderTrait {
    fn load(&mut self, reader: &mut File) -> io::Result<()>;

    ELFIO_GET_ACCESS_DECL!(u8, class);
    ELFIO_GET_ACCESS_DECL!(u8, elf_version);
    ELFIO_GET_ACCESS_DECL!(u8, encoding);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, header_size);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, section_entry_size);
    ELFIO_GET_ACCESS_DECL!(ElfHalf, segment_entry_size);

    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, version);
    ELFIO_GET_SET_ACCESS_DECL!(u8, os_abi);
    ELFIO_GET_SET_ACCESS_DECL!(u8, abi_version);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, type);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, machine);
    ELFIO_GET_SET_ACCESS_DECL!(ElfWord, flags);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Addr, entry);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, sections_num);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, sections_offset);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, segments_num);
    ELFIO_GET_SET_ACCESS_DECL!(Elf64Off, segments_offset);
    ELFIO_GET_SET_ACCESS_DECL!(ElfHalf, section_name_str_index);
}

// ELF file header
#[repr(C)]
#[derive(Debug)]
pub struct ElfHeader<Addr, Offset> {
    e_ident: [u8; EI_NIDENT],
    e_type: ElfHalf,
    e_machine: ElfHalf,
    e_version: ElfWord,
    e_entry: Addr,
    e_phoff: Offset,
    e_shoff: Offset,
    e_flags: ElfWord,
    e_ehsize: ElfHalf,
    e_phentsize: ElfHalf,
    e_phnum: ElfHalf,
    e_shentsize: ElfHalf,
    e_shnum: ElfHalf,
    e_shstrndx: ElfHalf,
}

impl<Addr, Offset> ElfHeader<Addr, Offset>
where
    Addr: Zero + NumCast + AsPrimitive<u64> + Num,
    Offset: Zero + NumCast + AsPrimitive<u64> + Num,
{
    pub fn new() -> ElfHeader<Addr, Offset> {
        ElfHeader::<Addr, Offset> {
            e_ident: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: Addr::zero(),
            e_phoff: Offset::zero(),
            e_shoff: Offset::zero(),
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }
}

impl<Addr, Offset> ElfHeaderTrait for ElfHeader<Addr, Offset>
where
    Addr: NumCast + Copy + AsPrimitive<u64>,
    Offset: NumCast + Copy + AsPrimitive<u64>,
{
    fn load(&mut self, reader: &mut File) -> io::Result<()> {
        let num_bytes = ::std::mem::size_of::<Self>();
        unsafe {
            let ptr = slice::from_raw_parts_mut(self as *mut Self as *mut u8, num_bytes);
            reader.read_exact(ptr)?;
        }
        Ok(())
    }

    ELFIO_GET_ACCESS!(u8, class, e_ident[EI_CLASS]);
    ELFIO_GET_ACCESS!(u8, elf_version, e_ident[EI_VERSION]);
    ELFIO_GET_ACCESS!(u8, encoding, e_ident[EI_DATA]);
    ELFIO_GET_ACCESS!(ElfHalf, header_size, e_ehsize);
    ELFIO_GET_ACCESS!(ElfHalf, section_entry_size, e_shentsize);
    ELFIO_GET_ACCESS!(ElfHalf, segment_entry_size, e_phentsize);

    ELFIO_GET_SET_ACCESS!(ElfWord, version, e_version);
    ELFIO_GET_SET_ACCESS!(u8, os_abi, e_ident[EI_OSABI]);
    ELFIO_GET_SET_ACCESS!(u8, abi_version, e_ident[EI_ABIVERSION]);
    ELFIO_GET_SET_ACCESS!(ElfHalf, type, e_type);
    ELFIO_GET_SET_ACCESS!(ElfHalf, machine, e_machine);
    ELFIO_GET_SET_ACCESS!(ElfWord, flags, e_flags);
    ELFIO_GET_SET_ACCESS!(ElfHalf, section_name_str_index, e_shstrndx);
    ELFIO_GET_SET_ACCESS!(Elf64Addr, entry, e_entry);
    ELFIO_GET_SET_ACCESS!(ElfHalf, sections_num, e_shnum);
    ELFIO_GET_SET_ACCESS!(Elf64Off, sections_offset, e_shoff);
    ELFIO_GET_SET_ACCESS!(ElfHalf, segments_num, e_phnum);
    ELFIO_GET_SET_ACCESS!(Elf64Off, segments_offset, e_phoff);
}
