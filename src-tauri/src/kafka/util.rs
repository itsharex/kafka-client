use std::io::{Cursor, BufRead};
use byteorder::{BigEndian, ReadBytesExt};
use std::str;

pub fn read_str<'a>(rdr: &'a mut Cursor<&[u8]>) -> Result<&'a str, Box<dyn std::error::Error>> {
    let len = (rdr.read_i16::<BigEndian>())? as usize;
    let pos = rdr.position() as usize;
    let slice = str::from_utf8(&rdr.get_ref()[pos..(pos + len)])?;
    rdr.consume(len);
    Ok(slice)
}

