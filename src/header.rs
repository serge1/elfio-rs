extern crate num;

use num::{NumCast, Zero};
use num::cast::AsPrimitive;
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
// pub const EI_VERSION: usize = 6;
// pub const EI_OSABI: usize = 7;
// pub const EI_ABIVERSION: usize = 8;
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

macro_rules! GET_SET_ACCESS_DECL {
    ($type: ident, $name: ident) => {
        paste! {
            fn [<get_ $name>](&self) -> $type;
        }
    };
}

macro_rules! GET_SET_ACCESS {
    ($type: ident, $name: ident, $field: ident) => {
        paste! {
            fn [<get_ $name>](&self) -> $type {
                paste! [self. $field].as_()
            }
        }
    };
}

pub trait ElfHeaderTrait {
    fn load(&mut self, buffer: &mut File) -> io::Result<()>;

    GET_SET_ACCESS_DECL!(u8, class);

    GET_SET_ACCESS_DECL!(ElfWord, version);
    // GET_SET_ACCESS_DECL!(u8, os_abi);
    // GET_SET_ACCESS_DECL!(u8, abi_version);
    GET_SET_ACCESS_DECL!(ElfHalf, type);
    GET_SET_ACCESS_DECL!(ElfHalf, machine);
    GET_SET_ACCESS_DECL!(ElfWord, flags);
    GET_SET_ACCESS_DECL!(Elf64Addr, entry);
    GET_SET_ACCESS_DECL!(ElfHalf, sections_num);
    GET_SET_ACCESS_DECL!(Elf64Off, sections_offset);
    GET_SET_ACCESS_DECL!(ElfHalf, segments_num);
    GET_SET_ACCESS_DECL!(Elf64Off, segments_offset);
    GET_SET_ACCESS_DECL!(ElfHalf, section_name_str_index);
}

impl<Addr, Offset> ElfHeader<Addr, Offset>
where
    Addr: Zero + NumCast + AsPrimitive<u64>,
    Offset: Zero + NumCast + AsPrimitive<u64>,
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
        println!("{}", num_bytes);
        unsafe {
            let ptr = slice::from_raw_parts_mut(self as *mut Self as *mut u8, num_bytes);
            reader.read_exact(ptr)?;
        }
        Ok(())
    }

    GET_SET_ACCESS!(ElfWord, version, e_version);
    // GET_SET_ACCESS!(u8, os_abi, e_ident[EI_OSABI]);
    // GET_SET_ACCESS!(u8, abi_version, e_ident[EI_ABIVERSION]);
    GET_SET_ACCESS!(ElfHalf, type, e_type);
    GET_SET_ACCESS!(ElfHalf, machine, e_machine);
    GET_SET_ACCESS!(ElfWord, flags, e_flags);
    GET_SET_ACCESS!(ElfHalf, section_name_str_index, e_shstrndx);
    GET_SET_ACCESS!(Elf64Addr, entry, e_entry);
    GET_SET_ACCESS!(ElfHalf, sections_num, e_shnum);
    GET_SET_ACCESS!(Elf64Off, sections_offset, e_shoff);
    GET_SET_ACCESS!(ElfHalf, segments_num, e_phnum);
    GET_SET_ACCESS!(Elf64Off, segments_offset, e_phoff);

    fn get_class(&self) -> u8 {
        self.e_ident[EI_CLASS]
    }
}
