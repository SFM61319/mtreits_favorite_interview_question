use bit_vec::BitVec;

use crate::{
    constants::U32_SIZE,
    stream::{read_chunks, write_chunk},
};

pub fn find_common(bin1: &str, bin2: &str, bin_out: &str) {
    let mut nums1 = BitVec::from_elem(U32_SIZE, false);
    read_chunks(bin1, |chunk| {
        let num = u32::from_le_bytes(*chunk);
        nums1.set(num as usize, true)
    })
    .unwrap();

    read_chunks(bin2, |chunk| {
        let num = u32::from_le_bytes(*chunk);
        if nums1[num as usize] {
            write_chunk(bin_out, chunk).unwrap();
            nums1.set(num as usize, false)
        }
    })
    .unwrap();
}
