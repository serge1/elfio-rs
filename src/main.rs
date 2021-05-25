use std::io;
use std::fs::File;

mod elfio;
mod header;
mod types;
mod utils;

pub use types::*;

fn main() -> io::Result<()> {
    // Eventually, change File to BufReader
    //let elf_file = File::open("/home/user/ELFIO/tests/elf_examples/hello_32")?;
    let elf_file = File::open("/home/user/elfio-rs/target/debug/elfio-rs")?;

    let mut elfio = elfio::Elfio::new();

    elfio.load(elf_file)?;

    Ok(())
}
