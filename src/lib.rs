#![warn(missing_docs)]

//! 'elfio' is a Rust library intended for reading and generation
//! files in the ELF binary format. The library supports processing
//! of ELF files for 32- and 64-bit architectures regardless of their
//! endianess
//!
//! For example:
//! ```
//! use std::fs::File;
//! use std::io;
//! use std::io::BufReader;
//!
//! use elfio::Elfio;
//!
//! fn main() -> io::Result<()> {
//!     let elf_file = File::open("tests/files/hello_64")?;
//!     let mut file_reader = BufReader::new(elf_file);
//!
//!     let mut elf = elfio::Elfio::new();
//!
//!     elf.load(&mut file_reader)?;
//!
//!     match elf.get_type() {
//!         elfio::constant::ET_REL => println!("Object ELF file"),
//!         elfio::constant::ET_EXEC => println!("Executable ELF file"),
//!         elfio::constant::ET_DYN => println!("Shared library ELF file"),
//!         elfio::constant::ET_CORE => println!("Core ELF file"),
//!         _ => println!("ELF type is not recognized"),
//!     }
//!
//!     Ok(())
//! }
//! ```

#[macro_use]
mod macros;

mod array;
mod dynamic;
mod elfio;
mod header;
mod modinfo;
mod note;
mod relocation;
mod section;
mod segment;
mod strings;
mod symbols;
mod types;
mod utils;

pub use crate::elfio::*;
pub use array::*;
pub use dynamic::*;
pub use modinfo::*;
pub use note::*;
pub use relocation::*;
pub use section::ElfSectionAccessTrait;
pub use segment::ElfSegmentAccessTrait;
pub use strings::*;
pub use symbols::*;
pub use types::*;
pub use utils::ElfioReadSeek;
