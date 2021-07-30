# ELFIO

**ELFIO** is a Rust library intended for reading and generating
files in the ELF binary format. The library supports processing
of ELF files for 32- and 64-bit architectures regardless of their
endianess

**ELFIO** is a Rust language port of the corresponding C++ library also
called [**ELFIO**](https://github.com/serge1/ELFIO)

## Status

Work in progress. Only ELF file reader is implemented so far.
Your contribution is welcomed!

## Documentation and Tutorial

Use **cargo** to produce the library documentation:

    cargo doc

Tutorial is available as an example source code. To compile the tutorial
use the following **cargo** command:

    cargo test --example tutorial

## Licensed under either of these

- MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

- Apache License, Version 2.0, (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
