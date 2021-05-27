use std::fs::File;
use std::io;
use std::io::prelude::*;
use paste::paste;

use super::header::*;
pub use super::types::*;

macro_rules! ELFIO_HEADER_ACCESS_GET {
    ($type: ident, $name: ident) => {
        paste! {
            pub fn [<get_ $name>](&self) -> $type {
                match &self.header {
                  Some(h) => h.[<get_ $name>](),
                  None => 0
                }
            }
        }
    };
}

macro_rules! ELFIO_HEADER_ACCESS_GET_SET {
    ($type: ident, $name: ident) => {
        paste! {
            pub fn [<get_ $name>](&self) -> $type {
                match &self.header {
                  Some(h) => h.[<get_ $name>](),
                  None => 0
                }
            }
            // fn [<set_ $name>](&mut self, value: $type) -> ();
        }
    };
}

pub struct Elfio {
    header: Option<Box<dyn ElfHeaderTrait>>,
}

impl Elfio {
    pub fn new() -> Elfio {
        Elfio { header: None }
    }

    pub fn load(&mut self, buffer: &mut File) -> io::Result<()> {
        let mut e_ident: [u8; EI_NIDENT] = [0; EI_NIDENT];
        // Read ELF file signature
        buffer.read_exact(&mut e_ident)?;
        buffer.seek(io::SeekFrom::Start(0))?;

        // Is it ELF file?
        if e_ident[EI_MAG0] != ELFMAG0
            || e_ident[EI_MAG1] != ELFMAG1
            || e_ident[EI_MAG2] != ELFMAG2
            || e_ident[EI_MAG3] != ELFMAG3
        {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File signature doesn't conform ELF file",
            ));
        }

        if e_ident[EI_CLASS] != ELFCLASS64 && e_ident[EI_CLASS] != ELFCLASS32 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown ELF class value",
            ));
        }

        if e_ident[EI_DATA] != ELFDATA2LSB && e_ident[EI_DATA] != ELFDATA2MSB {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Unknown ELF file endianess",
            ));
        }

        if e_ident[EI_CLASS] == ELFCLASS64 {
            self.header = Some(Box::new(ElfHeader::<Elf64Addr, Elf64Off>::new()));
        } else {
            self.header = Some(Box::new(ElfHeader::<Elf32Addr, Elf32Off>::new()));
        }

        match self.header.as_mut().unwrap().load(buffer) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }

    ELFIO_HEADER_ACCESS_GET!(u8, class);
    ELFIO_HEADER_ACCESS_GET!(u8, elf_version);
    ELFIO_HEADER_ACCESS_GET!(u8, encoding);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, header_size);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, section_entry_size);
    ELFIO_HEADER_ACCESS_GET!(ElfHalf, segment_entry_size);

    ELFIO_HEADER_ACCESS_GET_SET!(ElfWord, version);
    ELFIO_HEADER_ACCESS_GET_SET!(u8, os_abi);
    ELFIO_HEADER_ACCESS_GET_SET!(u8, abi_version);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, type);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, machine);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfWord, flags);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Addr, entry);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, sections_num);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Off, sections_offset);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, segments_num);
    ELFIO_HEADER_ACCESS_GET_SET!(Elf64Off, segments_offset);
    ELFIO_HEADER_ACCESS_GET_SET!(ElfHalf, section_name_str_index);
}
