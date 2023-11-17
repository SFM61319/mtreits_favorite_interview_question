use std::fs::File;
use std::io::{Read, Result};

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
