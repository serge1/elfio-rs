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
pub const ELFMAG1: u8 = b'E';
/// Identification value
pub const ELFMAG2: u8 = b'L';
/// Identification value
pub const ELFMAG3: u8 = b'F';

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
/// Operational system specific low index
pub const SHN_LOOS: ElfHalf = 0xFF20;
/// Operational system specific high index
pub const SHN_HIOS: ElfHalf = 0xFF3F;
/// This value specifies absolute values for the corresponding reference
pub const SHN_ABS: ElfHalf = 0xFFF1;
/// Symbols defined relative to this section are common symbols
pub const SHN_COMMON: ElfHalf = 0xFFF2;
/// This value specifies the upper bound of the range of reserved indexes.
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
/// This section contains an array of pointers to initialization functions
pub const SHT_INIT_ARRAY: ElfWord = 14;
/// This section contains an array of pointers to termination functions
pub const SHT_FINI_ARRAY: ElfWord = 15;
/// This section contains an array of pointers to functions that are invoked before all other initialization functions
pub const SHT_PREINIT_ARRAY: ElfWord = 16;
/// This section defines a section group
pub const SHT_GROUP: ElfWord = 17;
/// The section header index of the associated symbol table section
pub const SHT_SYMTAB_SHNDX: ElfWord = 18;
/// Values in this inclusive range are reserved for operating system-specific semantics
pub const SHT_LOOS: ElfWord = 0x60000000;
/// Values in this inclusive range are reserved for operating system-specific semantics
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
/// The data in the section may be merged to eliminate duplication
pub const SHF_MERGE: ElfXword = 0x10;
/// The data elements in the section consist of null-terminated character strings
pub const SHF_STRINGS: ElfXword = 0x20;
/// The sh_info field of this section header holds a section header table index
pub const SHF_INFO_LINK: ElfXword = 0x40;
/// This flag adds special ordering requirements for link editors
pub const SHF_LINK_ORDER: ElfXword = 0x80;
/// This section requires special OS-specific processing
pub const SHF_OS_NONCONFORMING: ElfXword = 0x100;
/// This section is a member (perhaps the only one) of a section group
pub const SHF_GROUP: ElfXword = 0x200;
/// This section holds Thread-Local Storage
pub const SHF_TLS: ElfXword = 0x400;
/// All bits included in this mask are reserved for operating system-specific semantics
pub const SHF_MASKOS: ElfXword = 0x0ff00000;
/// All bits included in this mask are reserved for processor-specific semantics
pub const SHF_MASKPROC: ElfXword = 0xF0000000;

// Section group flags
// pub const GRP_COMDAT   0x1
// pub const GRP_MASKOS   0x0ff00000
// pub const GRP_MASKPROC 0xf0000000

// Segment types
/// The array element is unused; other members' values are undefined
pub const PT_NULL: ElfWord = 0;
/// The array element specifies a loadable segment, described by p_filesz and p_memsz
pub const PT_LOAD: ElfWord = 1;
/// The array element specifies dynamic linking information
pub const PT_DYNAMIC: ElfWord = 2;
/// The array element specifies the location and size of a null-terminated path name to invoke as an interpreter
pub const PT_INTERP: ElfWord = 3;
/// The array element specifies the location and size of auxiliary information
pub const PT_NOTE: ElfWord = 4;
/// This segment type is reserved but has unspecified semantics
pub const PT_SHLIB: ElfWord = 5;
/// The array element, if present, specifies the location and size of the program header table itself
pub const PT_PHDR: ElfWord = 6;
/// The array element specifies the Thread-Local Storage template
pub const PT_TLS: ElfWord = 7;
/// Values in this inclusive range are reserved for operating system-specific semantics
pub const PT_LOOS: ElfWord = 0x60000000;
/// Values in this inclusive range are reserved for operating system-specific semantics
pub const PT_HIOS: ElfWord = 0x6fffffff;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const PT_LOPROC: ElfWord = 0x70000000;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const PT_HIPROC: ElfWord = 0x7FFFFFFF;

// Segment flags
/// Segment's Execute flag
pub const PF_X: ElfWord = 1;
/// Segment's Write flag
pub const PF_W: ElfWord = 2;
/// Segment's Read flag
pub const PF_R: ElfWord = 4;
/// Segment's Unspecified flag
pub const PF_MASKOS: ElfWord = 0x0ff00000;
/// Segment's Unspecified flag
pub const PF_MASKPROC: ElfWord = 0xf0000000;

// Symbol binding
/// Not visible outside the object file
pub const STB_LOCAL: u8 = 0;
/// Global symbol, visible to all object files
pub const STB_GLOBAL: u8 = 1;
/// Global scope, but with lower precedence than global symbols
pub const STB_WEAK: u8 = 2;
/// Environment-specific use
pub const STB_LOOS: u8 = 10;
/// Environment-specific use
pub const STB_HIOS: u8 = 12;
/// Processor-specific use
pub const STB_LOPROC: u8 = 13;
/// Processor-specific use
pub const STB_HIPROC: u8 = 15;

// Symbol types
/// No type specified (e.g., an absolute symbol)
pub const STT_NOTYPE: u8 = 0;
/// Data object
pub const STT_OBJECT: u8 = 1;
/// Function entry point
pub const STT_FUNC: u8 = 2;
/// Symbol is associated with a section
pub const STT_SECTION: u8 = 3;
/// Source file associated with the object file
pub const STT_FILE: u8 = 4;
/// The symbol labels a common block that has not yet been allocated.
pub const STT_COMMON: u8 = 5;
///
pub const STT_TLS: u8 = 6;
/// Environment-specific use
pub const STT_LOOS: u8 = 10;
/// Environment-specific use
pub const STT_HIOS: u8 = 12;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const STT_LOPROC: u8 = 13;
/// Values in this inclusive range are reserved for processor-specific semantics
pub const STT_HIPROC: u8 = 15;

// Dynamic Array Tags
/// Marks the end of the dynamic array
pub const DT_NULL: ElfSxword = 0;
/// The string table offset of the name of a needed library.
pub const DT_NEEDED: ElfSxword = 1;
/// Total size, in bytes, of the relocation entries associated with
/// the procedure linkage table
pub const DT_PLTRELSZ: ElfSxword = 2;
/// Contains an address associated with the linkage table. The
/// specific meaning of this field is processor-dependent.
pub const DT_PLTGOT: ElfSxword = 3;
/// Address of the symbol hash table, described below
pub const DT_HASH: ElfSxword = 4;
/// Address of the dynamic string table.
pub const DT_STRTAB: ElfSxword = 5;
/// Address of the dynamic symbol table
pub const DT_SYMTAB: ElfSxword = 6;
/// Address of a relocation table with Elfxx_Rela entries
pub const DT_RELA: ElfSxword = 7;
/// Total size, in bytes, of the DT_RELA relocation table
pub const DT_RELASZ: ElfSxword = 8;
/// Size, in bytes, of each DT_RELA relocation entry
pub const DT_RELAENT: ElfSxword = 9;
/// Total size, in bytes, of the string table
pub const DT_STRSZ: ElfSxword = 10;
/// Size, in bytes, of each symbol table entry
pub const DT_SYMENT: ElfSxword = 11;
/// Address of the initialization function
pub const DT_INIT: ElfSxword = 12;
/// Address of the termination function.
pub const DT_FINI: ElfSxword = 13;
/// The string table offset of the name of this shared object
pub const DT_SONAME: ElfSxword = 14;
/// The string table offset of a shared library search path string
pub const DT_RPATH: ElfSxword = 15;
/// The presence of this dynamic table entry modifies the
/// symbol resolution algorithm for references within the
/// library.
pub const DT_SYMBOLIC: ElfSxword = 16;
/// Address of a relocation table with Elfxx_Rel entries
pub const DT_REL: ElfSxword = 17;
/// Total size, in bytes, of the DT_REL relocation table
pub const DT_RELSZ: ElfSxword = 18;
/// Size, in bytes, of each DT_REL relocation entry.
pub const DT_RELENT: ElfSxword = 19;
/// Type of relocation entry used for the procedure linkage table
pub const DT_PLTREL: ElfSxword = 20;
/// Reserved for debugger use
pub const DT_DEBUG: ElfSxword = 21;
/// The presence of this dynamic table entry signals that the
/// relocation table contains relocations for a non-writable
/// segment
pub const DT_TEXTREL: ElfSxword = 22;
/// Address of the relocations associated with the procedure linkage table
pub const DT_JMPREL: ElfSxword = 23;
/// The presence of this dynamic table entry signals that the
/// dynamic loader should process all relocations for this object
/// before transferring control to the program
pub const DT_BIND_NOW: ElfSxword = 24;
/// Pointer to an array of pointers to initialization functions
pub const DT_INIT_ARRAY: ElfSxword = 25;
/// Pointer to an array of pointers to termination functions
pub const DT_FINI_ARRAY: ElfSxword = 26;
/// Size, in bytes, of the array of initialization functions
pub const DT_INIT_ARRAYSZ: ElfSxword = 27;
/// Size, in bytes, of the array of termination functions
pub const DT_FINI_ARRAYSZ: ElfSxword = 28;
///
pub const DT_RUNPATH: ElfSxword = 29;
///
pub const DT_FLAGS: ElfSxword = 30;
///
pub const DT_ENCODING: ElfSxword = 32;
///
pub const DT_PREINIT_ARRAY: ElfSxword = 32;
///
pub const DT_PREINIT_ARRAYSZ: ElfSxword = 33;
///
pub const DT_MAXPOSTAGS: ElfSxword = 34;
/// Defines a range of dynamic table tags that are reserved for
/// environment-specific use
pub const DT_LOOS: ElfSxword = 0x6000000D;
///
pub const DT_HIOS: ElfSxword = 0x6ffff000;
/// Defines a range of dynamic table tags that are reserved for
/// processor-specific use
pub const DT_LOPROC: ElfSxword = 0x70000000;
///
pub const DT_HIPROC: ElfSxword = 0x7FFFFFFF;
