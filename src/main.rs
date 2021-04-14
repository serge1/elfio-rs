mod elfio {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::mem;
    use std::slice;

    pub type ElfHalf = u16;
    pub type ElfWord = u32;
    pub type ElfSword = i32;
    pub type ElfXword = u64;
    pub type ElfSxword = i64;

    pub type Elf32Addr = u32;
    pub type Elf32Off = u32;
    pub type Elf64Addr = u64;
    pub type Elf64Off = u64;

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
    pub const ELFCLASSNONE: u8 = 0;
    pub const ELFCLASS32: u8 = 1;
    pub const ELFCLASS64: u8 = 2;

    // Encoding
    pub const ELFDATANONE: u8 = 0;
    pub const ELFDATA2LSB: u8 = 1;
    pub const ELFDATA2MSB: u8 = 2;

    // ELF file header
    #[repr(C)]
    #[derive(Debug)]
    struct ElfEhdr<Addr, Offset> {
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

    enum ElfHeader {
        ElfHeader32(ElfEhdr<Elf32Addr, Elf32Off>),
        ElfHeader64(ElfEhdr<Elf64Addr, Elf64Off>),
    }

    pub struct Elfio {
        header: Option<ElfHeader>,
    }

    impl Elfio {
        pub fn new() -> Elfio {
            return Elfio { header: None };
        }

        fn read_struct<T, R: Read>(read: &mut R) -> io::Result<T> {
            let num_bytes = ::std::mem::size_of::<T>();
            unsafe {
                let mut s = ::std::mem::uninitialized();
                let buffer = slice::from_raw_parts_mut(&mut s as *mut T as *mut u8, num_bytes);
                match read.read_exact(buffer) {
                    Ok(()) => Ok(s),
                    Err(e) => {
                        ::std::mem::forget(s);
                        Err(e)
                    }
                }
            }
        }

        pub fn load(&mut self, buffer: &mut File) -> Result<(), &'static str> {
            let mut e_ident: [u8; EI_NIDENT] = [0; EI_NIDENT];
            // Read ELF file signature
            buffer
                .read_exact(&mut e_ident)
                .expect("Can't read from file");

            match buffer.seek(io::SeekFrom::Start(0)) {
                Ok(s) => s,
                Err(_) => return Err("File operation failed"),
            };

            // Is it ELF file?
            if e_ident[EI_MAG0] != ELFMAG0
                || e_ident[EI_MAG1] != ELFMAG1
                || e_ident[EI_MAG2] != ELFMAG2
                || e_ident[EI_MAG3] != ELFMAG3
            {
                return Err("File signature doesn't conform ELF file");
            }

            if e_ident[EI_CLASS] != ELFCLASS64 && e_ident[EI_CLASS] != ELFCLASS32 {
                return Err("Unknown ELF class value");
            }

            if e_ident[EI_DATA] != ELFDATA2LSB && e_ident[EI_DATA] != ELFDATA2MSB {
                return Err("Unknown ELF file endianess");
            }

            // let mut header: ElfEhdr<Elf64Addr, Elf64Off> =
            //     Self::read_struct::<ElfEhdr<Elf64Addr, Elf64Off>, File>(buffer).unwrap();

            if e_ident[EI_CLASS] == ELFCLASS64 {
                let header = match Self::read_struct::<ElfEhdr<Elf64Addr, Elf64Off>, File>(buffer) {
                    Ok(h) => h,
                    Err(_) => return Err("File operation failed"),
                };
                self.header = Some(ElfHeader::ElfHeader64(header));
            } else {
                let header = match Self::read_struct::<ElfEhdr<Elf32Addr, Elf32Off>, File>(buffer) {
                    Ok(h) => h,
                    Err(_) => return Err("File operation failed"),
                };
                self.header = Some(ElfHeader::ElfHeader32(header));
            }

            match &self.header {
                Some(ElfHeader::ElfHeader64(x)) => println!("64-bit header: {:#?}", x),
                Some(ElfHeader::ElfHeader32(x)) => println!("32-bit header: {:#?}", x),
                None => println!("There is no header"),
            }

            return Ok(());
        }
    }
}

use std::io;

fn main() -> io::Result<()> {
    use std::fs::File;

    // Eventually, change it to BufReader
    let mut file = File::open("/home/user/ELFIO/tests/elf_examples/hello_32")?;

    let mut elfio = elfio::Elfio::new();

    match elfio.load(&mut file) {
        Ok(_) => println!("It is OK"),
        Err(str) => println!("Error: {}", str),
    }

    Ok(())
}
