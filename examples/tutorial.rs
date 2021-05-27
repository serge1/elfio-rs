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

use std::fs::File;
use std::io;

use elfio::*;

fn main() -> io::Result<()> {
    let mut elf_file = File::open("tests/files/hello_64")?;

    let mut reader = Elfio::new();

    reader.load(&mut elf_file)?;

    match reader.get_class() {
        elfio::ELFCLASS32 => println!("32-bit ELF file"),
        elfio::ELFCLASS64 => println!("64-bit ELF file"),
        _ => println!("ELF class is not recognized")
    }

    match reader.get_encoding() {
        elfio::ELFDATA2LSB => println!("LSB ELF file"),
        elfio::ELFDATA2MSB => println!("MSB ELF file"),
        _ => println!("ELF endianess is not recognized")
    }

    println!("Start address: 0x{:08X}", reader.get_entry());

    Ok(())
}
