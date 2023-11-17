use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

use crate::constants::CHUNK_SIZE;

pub fn read_chunks<F>(filename: &str, mut f: F) -> Result<()>
where
    F: FnMut(&[u8; CHUNK_SIZE]),
{
    let mut file = File::open(filename)?;
    let mut buffer = [u8::MIN; CHUNK_SIZE];

    loop {
        let bytes = file.read(&mut buffer)?;
        if bytes == usize::MIN {
            break;
        }

        f(&buffer);
    }

    Ok(())
}

pub fn write_chunk(filename: &str, data: &[u8; CHUNK_SIZE]) -> Result<()> {
    let _bytes = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?
        .write(data)?;

    Ok(())
}
