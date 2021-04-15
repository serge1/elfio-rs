mod elfio {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::slice;

    pub mod types;
    pub use types::*;

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
        e_type: types::ElfHalf,
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

    trait ElfHeaderTrait {
        fn load(&self) -> Result<(), &'static str>;
    }

    enum ElfHeader {
        ElfHeader32(ElfEhdr<Elf32Addr, Elf32Off>),
        ElfHeader64(ElfEhdr<Elf64Addr, Elf64Off>),
    }

    impl ElfHeaderTrait for ElfHeader {
        fn load(&self) -> Result<(), &'static str> {
            println!("'load' has been called");

            match &self {
                ElfHeader::ElfHeader64(_) => println!("64-bit header load"),
                ElfHeader::ElfHeader32(_) => println!("32-bit header load"),
            }

            return Ok(());
        }
    }

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
                let header = Self::read_struct::<ElfEhdr<Elf64Addr, Elf64Off>, File>(buffer)?;
                self.header = Some(Box::new(ElfHeader::ElfHeader64(header)));
            } else {
                let header = Self::read_struct::<ElfEhdr<Elf32Addr, Elf32Off>, File>(buffer)?;
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
}

use std::io;

fn main() -> io::Result<()> {
    use std::fs::File;

    // Eventually, change File to BufReader
    //    let mut file = File::open("/home/user/ELFIO/tests/elf_examples/hello_32")?;
    let file = File::open("/home/user/elfio-rs/target/debug/elfio-rs")?;

    let mut elfio = elfio::Elfio::new();

    elfio.load(file)?;

    Ok(())
}
