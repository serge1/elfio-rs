use std::io;

mod elfio;
mod header;
mod types;

fn main() -> io::Result<()> {
    use std::fs::File;

    // Eventually, change File to BufReader
    //    let mut file = File::open("/home/user/ELFIO/tests/elf_examples/hello_32")?;
    let file = File::open("/home/user/elfio-rs/target/debug/elfio-rs")?;

    let mut reader = elfio::Elfio::new();

    reader.load(file)?;

    Ok(())
}
