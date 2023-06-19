mod builder;
mod iterator;

pub use builder::BlockBuilder;
use bytes::{Buf, BufMut, Bytes};
pub use iterator::BlockIterator;

pub const U16_SIZE: usize = std::mem::size_of::<u16>();

/// A block is the smallest unit of read and caching in LSM tree. It is a collection of sorted
/// key-value pairs.
pub struct Block {
    data: Vec<u8>,
    offsets: Vec<u16>,
}

impl Block {
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        let offset_len = self.offsets.len();
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        buf.put_u16(offset_len as u16);
        buf.into()
    }

    pub fn decode(data: &[u8]) -> Self {
        let entry_offset_len = (&data[data.len() - U16_SIZE..]).get_u16() as usize;
        let data_end = data.len() - U16_SIZE - entry_offset_len * U16_SIZE;
        let offset_raw = &data[data_end..data.len() - U16_SIZE];
        let offset = offset_raw
            .chunks(U16_SIZE)
            .map(|mut x| x.get_u16())
            .collect();
        let data = data[0..data_end].to_vec();
        Self {
            data,
            offsets: offset,
        }
    }
}

#[cfg(test)]
mod tests;
