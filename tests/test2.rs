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

use elfio::*;

#[test]
fn sym_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".symtab") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x44);
    // Num:    Value  Size Type    Bind   Vis      Ndx Name
    //  30: 08049588     4 OBJECT  LOCAL  DEFAULT   23 dtor_idx.5805
    let sym = symtab.get_symbol(30).unwrap();
    assert_eq!(sym.value, 0x08049588);
    assert_eq!(sym.size, 4);
    assert_eq!(sym.bind, STB_LOCAL);
    assert_eq!(sym.stype, STT_OBJECT);
    assert_eq!(sym.shndx, 23);
    assert_eq!(sym.name, "dtor_idx.5805");

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
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x43);

    let sym = symtab.get_symbol(33).unwrap();
    // 33: 0000000000400410     0 FUNC    LOCAL  DEFAULT   12 __do_global_dtors_aux
    assert_eq!(sym.value, 0x400410);
    assert_eq!(sym.size, 0);
    assert_eq!(sym.bind, STB_LOCAL);
    assert_eq!(sym.stype, STT_FUNC);
    assert_eq!(sym.shndx, 12);
    assert_eq!(sym.name, "__do_global_dtors_aux");

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
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x50);

    let sym = symtab.get_symbol(34).unwrap();
    // 34: 10010c98     1 OBJECT  LOCAL  DEFAULT   24 completed.6348
    assert_eq!(sym.value, 0x10010c98);
    assert_eq!(sym.size, 1);
    assert_eq!(sym.bind, STB_LOCAL);
    assert_eq!(sym.stype, STT_OBJECT);
    assert_eq!(sym.shndx, 24);
    assert_eq!(sym.name, "completed.6348");

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
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let symtab = SymbolSectionAccessor::new(&elf, &*section);
    assert_eq!(symtab.get_symbols_num(), 0x19);

    let sym = symtab.get_symbol(24).unwrap();
    // 24: 000000000001fc18   108 FUNC    GLOBAL DEFAULT   21 error
    assert_eq!(sym.value, 0x1fc18);
    assert_eq!(sym.size, 108);
    assert_eq!(sym.bind, STB_GLOBAL);
    assert_eq!(sym.stype, STT_FUNC);
    assert_eq!(sym.shndx, 21);
    assert_eq!(sym.name, "error");

    Ok(())
}

#[test]
fn note_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".note.ABI-tag") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let notes = NoteSectionAccessor::new(&elf, section);

    assert_eq!(notes.get_notes_num(), 1);

    let note = notes.get_note(0).unwrap();
    assert_eq!(note.ntype, 1);
    assert_eq!(note.name, "GNU");
    assert_eq!(
        note.description,
        vec![0u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 6u8, 0u8, 0u8, 0u8, 9u8, 0u8, 0u8, 0u8]
    );

    Ok(())
}

#[test]
fn note_le_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".note.ABI-tag") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let notes = NoteSectionAccessor::new(&elf, section);

    assert_eq!(notes.get_notes_num(), 1);

    let note = notes.get_note(0).unwrap();
    assert_eq!(note.ntype, 1);
    assert_eq!(note.name, "GNU");
    assert_eq!(
        note.description,
        vec![0, 0, 0, 0, 2, 0, 0, 0, 6, 0, 0, 0, 9, 0, 0, 0]
    );

    Ok(())
}

#[test]
fn note_be_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".note.ABI-tag") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let notes = NoteSectionAccessor::new(&elf, section);

    assert_eq!(notes.get_notes_num(), 1);

    let note = notes.get_note(0).unwrap();
    assert_eq!(note.ntype, 1);
    assert_eq!(note.name, "GNU");
    assert_eq!(
        note.description,
        vec![0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 6, 0, 0, 0, 10]
    );

    Ok(())
}

#[test]
fn note_be_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".note.ABI-tag") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let notes = NoteSectionAccessor::new(&elf, section);

    assert_eq!(notes.get_notes_num(), 1);

    let note = notes.get_note(0).unwrap();
    assert_eq!(note.ntype, 1);
    assert_eq!(note.name, "GNU");
    assert_eq!(
        note.description,
        vec![0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 0]
    );

    Ok(())
}

#[test]
fn rel_le_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_32")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".rel.plt") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let relocs = RelocationSectionAccessor::new(&elf, section);

    assert_eq!(relocs.get_entries_num(), 3);

    // 0804957c  00000207 R_386_JUMP_SLOT   00000000   __libc_start_main@GLIBC_2.0
    let rel = relocs.get_entry(1).unwrap();
    assert_eq!(rel.offset, 0x0804957C);
    assert_eq!(rel.symbol, 2);
    assert_eq!(rel.rtype, 7);
    assert_eq!(rel.addend, None);

    Ok(())
}

#[test]
fn rel_le_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".rela.plt") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let relocs = RelocationSectionAccessor::new(&elf, section);

    assert_eq!(relocs.get_entries_num(), 2);

    // 000000600850  000300000007 R_X86_64_JUMP_SLO 0000000000000000 __libc_start_main@GLIBC_2.2.5 + 0
    let rel = relocs.get_entry(1).unwrap();
    assert_eq!(rel.offset, 0x000000600850);
    assert_eq!(rel.symbol, 3);
    assert_eq!(rel.rtype, 7);
    assert_eq!(rel.addend, Some(0));

    Ok(())
}

#[test]
fn rel_be_32() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".rela.plt") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let relocs = RelocationSectionAccessor::new(&elf, section);

    assert_eq!(relocs.get_entries_num(), 9);

    // 10010bf4  00000615 R_PPC_JMP_SLOT    10000920   _ZNSt8ios_base4In[...]@GLIBCXX_3.4 + 0
    let rel = relocs.get_entry(4).unwrap();
    assert_eq!(rel.offset, 0x10010bf4);
    assert_eq!(rel.symbol, 6);
    assert_eq!(rel.rtype, 0x15);
    assert_eq!(rel.addend, Some(0));

    Ok(())
}

#[test]
fn rel_be_64() -> io::Result<()> {
    let elf_file = File::open("tests/files/hello_ppc64")?;
    let mut reader = BufReader::new(elf_file);

    let mut elf = Elfio::new();

    elf.load(&mut reader)?;

    let section = match elf.get_section_by_name(&".rela.plt") {
        Some(s) => s,
        None => return Err(Error::new(io::ErrorKind::Other, "section not found")),
    };

    let relocs = RelocationSectionAccessor::new(&elf, section);

    assert_eq!(relocs.get_entries_num(), 17);

    // 0000001fee8  001700000015 R_PPC64_JMP_SLOT  0000000000000000 exit@GLIBC_2.3 + 0
    let rel = relocs.get_entry(16).unwrap();
    assert_eq!(rel.offset, 0x1fee8);
    assert_eq!(rel.symbol, 0x17);
    assert_eq!(rel.rtype, 0x15);
    assert_eq!(rel.addend, Some(0));

    Ok(())
}
