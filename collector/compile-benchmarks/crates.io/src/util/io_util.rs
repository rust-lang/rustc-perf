use std::io;
use std::io::prelude::*;
use std::mem;

pub fn read_le_u32<R: Read + ?Sized>(r: &mut R) -> io::Result<u32> {
    let mut b = [0; 4];
    read_fill(r, &mut b)?;
    Ok(
        u32::from(b[0])
            | (u32::from(b[1]) << 8)
            | (u32::from(b[2]) << 16)
            | (u32::from(b[3]) << 24),
    )
}

pub fn read_fill<R: Read + ?Sized>(r: &mut R, mut slice: &mut [u8]) -> io::Result<()> {
    while !slice.is_empty() {
        let n = r.read(slice)?;
        if n == 0 {
            return Err(io::Error::other("end of file reached"));
        }
        slice = &mut mem::take(&mut slice)[n..];
    }
    Ok(())
}
