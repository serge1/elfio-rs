use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::slice;

use crate::types::*;
use crate::header::*;


pub struct Elfio {
    header: Option<Box<dyn ElfHeaderTrait>>,
}

impl Elfio {
    pub fn new() -> Elfio {
        return Elfio { header: None };
    }

    fn read_struct<T, R: Read>(buffer: &mut R) -> io::Result<T> {
        let num_bytes = ::std::mem::size_of::<T>();
        unsafe {
            let mut mem = ::std::mem::MaybeUninit::uninit().assume_init();
            let ptr = slice::from_raw_parts_mut(&mut mem as *mut T as *mut u8, num_bytes);
            match buffer.read_exact(ptr) {
                Ok(()) => Ok(mem),
                Err(e) => {
                    ::std::mem::forget(mem);
                    Err(e)
                }
            }
        }
    }

    fn create_and_load_header(&mut self, buffer: &mut File, class: u8) -> io::Result<()> {
        if class == ELFCLASS64 {
            let header = Self::read_struct::<
                ElfEhdr<Elf64Addr, Elf64Off>,
                File,
            >(buffer)?;
            self.header = Some(Box::new(ElfHeader::ElfHeader64(header)));
        } else {
            let header = Self::read_struct::<
                ElfEhdr<Elf32Addr, Elf32Off>,
                File,
            >(buffer)?;
            self.header = Some(Box::new(ElfHeader::ElfHeader32(header)));
        }

        Ok(())
    }

    pub fn load(&mut self, mut buffer: File) -> io::Result<()> {
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

        self.create_and_load_header(&mut buffer, e_ident[EI_CLASS])
    }
}
