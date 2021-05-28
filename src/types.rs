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
