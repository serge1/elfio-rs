/*
Copyright (C) 2021-present by Serge Lamikhov-Center

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

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
