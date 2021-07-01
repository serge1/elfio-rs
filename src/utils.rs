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

use num_traits::AsPrimitive;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

// --------------------------------------------------------------------------
macro_rules! impl_load_for {
    ( $x:ty ) => {
        impl Load for $x {
            fn load(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
                let mut buffer = self.to_ne_bytes();
                reader.read_exact(&mut buffer)?;
                *self = <$x>::from_ne_bytes(buffer);

                Ok(())
            }
        }
    };
}

// --------------------------------------------------------------------------
/// The trait for (de)serializing ELF entities
pub trait Load {
    fn load(&mut self, reader: &mut BufReader<File>) -> io::Result<()>;
}

// --------------------------------------------------------------------------
impl_load_for!(u8);
impl_load_for!(u16);
impl_load_for!(u32);
impl_load_for!(u64);

impl Load for &mut [u8; 16] {
    fn load(&mut self, reader: &mut BufReader<File>) -> io::Result<()> {
        reader.read_exact(*self)?;

        Ok(())
    }
}

// --------------------------------------------------------------------------
macro_rules! impl_convert_for {
    ( $x:ty ) => {
        #[cfg(target_endian = "little")]
        impl Convert<$x> for Converter {
            fn convert(&self, value: $x) -> $x {
                if self.is_needed {
                    value.to_be()
                } else {
                    value
                }
            }
        }
        #[cfg(target_endian = "big")]
        impl Convert<$x> for Converter {
            fn convert(&self, value: $x) -> $x {
                if self.is_needed {
                    value.to_le()
                } else {
                    value
                }
            }
        }
    };
}

// --------------------------------------------------------------------------
pub trait Convert<T>
where
    T: AsPrimitive<u64>,
{
    fn convert(&self, value: T) -> T;
}

// --------------------------------------------------------------------------
#[derive(Debug, Copy, Clone)]
pub struct Converter {
    pub is_needed: bool,
}

// --------------------------------------------------------------------------
impl_convert_for!(u8);
impl_convert_for!(u16);
impl_convert_for!(u32);
impl_convert_for!(u64);


// --------------------------------------------------------------------------
#[test]
fn test_conv() -> () {
    let conv = Converter { is_needed: true };

    let a = 0x12u8;
    let a = conv.convert(a);
    let b = 0x1234u16;
    let b = conv.convert(b);
    let c = 0x12345678u32;
    let c = conv.convert(c);
    let d = 0x1234567890ABCDEFu64;
    let d = conv.convert(d);

    assert_eq!(a, 0x12);
    assert_eq!(b, 0x3412);
    assert_eq!(c, 0x78563412);
    assert_eq!(d, 0xEFCDAB9078563412);
}

// --------------------------------------------------------------------------
#[test]
fn test_no_conv() -> () {
    let conv = Converter { is_needed: false };

    let a = 0x12u8;
    let a = conv.convert(a);
    let b = 0x1234u16;
    let b = conv.convert(b);
    let c = 0x12345678u32;
    let c = conv.convert(c);
    let d = 0x1234567890ABCDEFu64;
    let d = conv.convert(d);

    assert_eq!(a, 0x12);
    assert_eq!(b, 0x1234);
    assert_eq!(c, 0x12345678);
    assert_eq!(d, 0x1234567890ABCDEF);
}
