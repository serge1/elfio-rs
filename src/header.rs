use std::fs::File;
use std::io;

use crate::types::*;
use super::utils::*;

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
pub const EI_PAD: usize = 9;
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
pub struct ElfEhdr<Addr, Offset> {
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

pub enum ElfHeader {
    ElfHeader32(ElfEhdr<Elf32Addr, Elf32Off>),
    ElfHeader64(ElfEhdr<Elf64Addr, Elf64Off>),
}

pub fn load(buffer: &mut File, class: u8) -> io::Result<Option<Box<dyn ElfHeaderTrait>>> {
    if class == ELFCLASS64 {
        let header = super::utils::read_struct::<ElfEhdr<Elf64Addr, Elf64Off>, File>(buffer);
        match header {
            Ok(h) => return Ok(Some(Box::new(ElfHeader::ElfHeader64(h)))),
            Err(e) => return Err(e),
        }
    } else {
        let header = super::utils::read_struct::<ElfEhdr<Elf32Addr, Elf32Off>, File>(buffer);
        match header {
            Ok(h) => return Ok(Some(Box::new(ElfHeader::ElfHeader32(h)))),
            Err(e) => return Err(e),
        }
    }
}

pub trait ElfHeaderTrait {
    fn whoami(&self);
    fn get_class(&self) -> ElfHalf;
}

impl ElfHeaderTrait for ElfHeader {
    fn whoami(&self) {
        match self {
            ElfHeader::ElfHeader32(_) => println!("32-bit ELF header"),
            ElfHeader::ElfHeader64(_) => println!("64-bit ELF header"),
        }
    }

    fn get_class(&self) -> ElfHalf {
        match self {
            ElfHeader::ElfHeader32(s) => {
                println!("e_shstrndx {}", s.e_shstrndx);
                return s.e_shstrndx;
            }
            ElfHeader::ElfHeader64(s) => {
                println!("e_shstrndx {}", s.e_shstrndx);
                return s.e_shstrndx;
            }
        }
    }
}
