use std::fs::File;
use std::io;
use std::io::prelude::*;

use super::header::*;
pub use super::types::*;

pub struct Elfio {
    header: Option<Box<dyn ElfHeaderTrait>>,
}

impl Elfio {
    pub fn new() -> Elfio {
        return Elfio { header: None };
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

        match super::header::load(&mut buffer, e_ident[EI_CLASS]) {
            Ok(h) => self.header = h,
            Err(e) => return Err(e),
        }

        self.header.as_ref().unwrap().whoami();
        self.header.as_ref().unwrap().get_class();

        return Ok(());
    }
}
