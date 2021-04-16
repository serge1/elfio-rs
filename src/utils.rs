use std::io;
use std::io::prelude::*;
use std::slice;

pub fn read_struct<T, R: Read>(buffer: &mut R) -> io::Result<T> {
    let num_bytes = ::std::mem::size_of::<T>();
    unsafe {
        let mut mem = ::std::mem::MaybeUninit::uninit().assume_init();
        let ptr = slice::from_raw_parts_mut(&mut mem as *mut T as *mut u8, num_bytes);
        match buffer.read_exact(ptr) {
            Ok(()) => Ok(mem),
            Err(e) => {
                ::std::mem::forget(mem);
                Err(e)
            }
        }
    }
}
