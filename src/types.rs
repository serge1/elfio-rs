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

/// Unsigned medium integer
pub type ElfHalf = u16;
/// Unsigned large integer
pub type ElfWord = u32;
/// Signed large integer
pub type ElfSword = i32;
/// Unsigned long integer
pub type ElfXword = u64;
/// Signed long integer
pub type ElfSxword = i64;

/// Unsigned program address for 32-bit
pub type Elf32Addr = u32;
/// Unsigned file offset for 32-bit
pub type Elf32Off = u32;
/// Unsigned program address for 64-bit
pub type Elf64Addr = u64;
/// Unsigned file offset for 64-bit
pub type Elf64Off = u64;

// File version
/// Invalid version
pub const EV_NONE: u8 = 0;
/// Current version
pub const EV_CURRENT: u8 = 1;

// Identification index
/// File identification
pub const EI_MAG0: usize = 0;
/// File identification
pub const EI_MAG1: usize = 1;
/// File identification
pub const EI_MAG2: usize = 2;
/// File identification
pub const EI_MAG3: usize = 3;
/// File class
pub const EI_CLASS: usize = 4;
/// Data encoding
pub const EI_DATA: usize = 5;
/// File version
pub const EI_VERSION: usize = 6;
/// OS/ABI identification
pub const EI_OSABI: usize = 7;
/// ABI version
pub const EI_ABIVERSION: usize = 8;
/// Start of padding bytes
pub const EI_PAD: usize = 9;
/// Size of e_ident[]
pub const EI_NIDENT: usize = 16;

// Magic number
/// Identification value
pub const ELFMAG0: u8 = 0x7F;
/// Identification value
pub const ELFMAG1: u8 = 'E' as u8;
/// Identification value
pub const ELFMAG2: u8 = 'L' as u8;
/// Identification value
pub const ELFMAG3: u8 = 'F' as u8;

// File class
/// Invalid class
pub const ELFCLASSNONE: u8 = 0;
/// 32-bit objects
pub const ELFCLASS32: u8 = 1;
/// 64-bit objects
pub const ELFCLASS64: u8 = 2;

// Encoding
/// Invalid data encoding
pub const ELFDATANONE: u8 = 0;
/// The least significant byte occupying the lowest address
pub const ELFDATA2LSB: u8 = 1;
/// The most significant byte occupying the lowest address
pub const ELFDATA2MSB: u8 = 2;

// File types
/// No file type
pub const ET_NONE: ElfHalf = 0;
/// Relocatable object file
pub const ET_REL: ElfHalf = 1;
/// Executable file
pub const ET_EXEC: ElfHalf = 2;
/// Shared object file
pub const ET_DYN: ElfHalf = 3;
/// Core file
pub const ET_CORE: ElfHalf = 4;

// Section indexes
/// This value marks an undefined, missing, irrelevant, or otherwise
/// meaningless section reference.
pub const SHN_UNDEF: ElfHalf = 0;
/// This value specifies the lower bound of the range of reserved indexes
pub const SHN_LORESERVE: ElfHalf = 0xFF00;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const SHN_LOPROC: ElfHalf = 0xFF00;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const SHN_HIPROC: ElfHalf = 0xFF1F;
pub const SHN_LOOS: ElfHalf = 0xFF20;
pub const SHN_HIOS: ElfHalf = 0xFF3F;
/// This value specifies absolute values for the corresponding reference
pub const SHN_ABS: ElfHalf = 0xFFF1;
/// Symbols defined relative to this section are common symbols
pub const SHN_COMMON: ElfHalf = 0xFFF2;
pub const SHN_XINDEX: ElfHalf = 0xFFFF;
/// This value specifies the upper bound of the range of reserved indexes.
pub const SHN_HIRESERVE: ElfHalf = 0xFFFF;

// Section types
/// This value marks the section header as inactive; it does not have an
/// associated section
pub const SHT_NULL: ElfWord = 0;
/// The section holds information defined by the program, whose format and
/// meaning are determined solely by the program
pub const SHT_PROGBITS: ElfWord = 1;
/// These sections hold a symbol table
pub const SHT_SYMTAB: ElfWord = 2;
/// The section holds a string table
pub const SHT_STRTAB: ElfWord = 3;
/// The section holds relocation entries with explicit addends
pub const SHT_RELA: ElfWord = 4;
/// The section holds a symbol hash table
pub const SHT_HASH: ElfWord = 5;
/// The section holds information for dynamic linking
pub const SHT_DYNAMIC: ElfWord = 6;
/// This section holds information that marks the file in some way
pub const SHT_NOTE: ElfWord = 7;
/// A section of this type occupies no space in the file but otherwise resembles
/// SHT_PROGBITS
pub const SHT_NOBITS: ElfWord = 8;
/// The section holds relocation entries without explicit addends
pub const SHT_REL: ElfWord = 9;
/// This section type is reserved but has unspecified semantics
pub const SHT_SHLIB: ElfWord = 10;
/// These sections hold a symbol table.
pub const SHT_DYNSYM: ElfWord = 11;
pub const SHT_INIT_ARRAY: ElfWord = 14;
pub const SHT_FINI_ARRAY: ElfWord = 15;
pub const SHT_PREINIT_ARRAY: ElfWord = 16;
pub const SHT_GROUP: ElfWord = 17;
pub const SHT_SYMTAB_SHNDX: ElfWord = 18;
pub const SHT_LOOS: ElfWord = 0x60000000;
pub const SHT_HIOS: ElfWord = 0x6fffffff;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const SHT_LOPROC: ElfWord = 0x70000000;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const SHT_HIPROC: ElfWord = 0x7FFFFFFF;
/// This value specifies the lower bound of the range of indexes reserved for
/// application programs
pub const SHT_LOUSER: ElfWord = 0x80000000;
/// This value specifies the upper bound of the range of indexes reserved for
/// application programs
pub const SHT_HIUSER: ElfWord = 0xFFFFFFFF;

// Section attribute flags
/// The section contains data that should be writable during process execution
pub const SHF_WRITE: ElfXword = 0x1;
/// The section occupies memory during process execution
pub const SHF_ALLOC: ElfXword = 0x2;
/// The section contains executable machine instructions
pub const SHF_EXECINSTR: ElfXword = 0x4;
///
pub const SHF_MERGE: ElfXword = 0x10;
pub const SHF_STRINGS: ElfXword = 0x20;
pub const SHF_INFO_LINK: ElfXword = 0x40;
pub const SHF_LINK_ORDER: ElfXword = 0x80;
pub const SHF_OS_NONCONFORMING: ElfXword = 0x100;
pub const SHF_GROUP: ElfXword = 0x200;
pub const SHF_TLS: ElfXword = 0x400;
pub const SHF_MASKOS: ElfXword = 0x0ff00000;
/// All bits included in this mask are reserved for processor-specific semantics
pub const SHF_MASKPROC: ElfXword = 0xF0000000;

// Section group flags
// pub const GRP_COMDAT   0x1
// pub const GRP_MASKOS   0x0ff00000
// pub const GRP_MASKPROC 0xf0000000
