pub mod elfio;
mod header;
mod types;
mod utils;

//pub use types::*;

// fn main() -> io::Result<()> {
//     // Eventually, change File to BufReader
//     //let elf_file = File::open("/home/user/ELFIO/tests/elf_examples/hello_32")?;
//     let elf_file = File::open("/home/user/elfio-rs/target/debug/elfio-rs")?;

//     let mut elfio = elfio::Elfio::new();

//     elfio.load(elf_file)?;

//     let v1 = elfio.header.as_ref().unwrap().get_class();
//     let v2 = elfio.header.as_ref().unwrap().get_sections_num();
//     let v3 = elfio.header.as_ref().unwrap().get_section_name_str_index();
//     let v4 = elfio.header.as_ref().unwrap().get_entry();

//     println!("{} {} {} {}", v1, v2, v3, v4);

//     Ok(())
// }
