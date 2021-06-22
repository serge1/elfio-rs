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
use std::io::{BufReader, Error};

use elfio::{Elfio, SymbolSectionAccessor};

#[test]
fn sym_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".symtab") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found"))
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x44);

    Ok(())
}

#[test]
fn sym_le_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".symtab") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found"))
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x43);

    let sym = symtab.get_symbol(33).unwrap();
    assert_eq!(sym.value, 0x400410);
    assert_eq!(sym.size, 0);
    assert_eq!(sym.shndx, 12);

    Ok(())
}

#[test]
fn sym_be_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".symtab") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found"))
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x50);

    Ok(())
}

#[test]
fn sym_be_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".dynsym") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found"))
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x19);

    Ok(())
}
