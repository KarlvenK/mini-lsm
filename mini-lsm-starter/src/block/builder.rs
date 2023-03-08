use super::Block;
use crate::block::U16_SIZE;
use bytes::BufMut;

/// Builds a block.
pub struct BlockBuilder {
    offset: Vec<u16>,
    data: Vec<u8>,
    block_size: usize,
}

impl BlockBuilder {
    /// Creates a new block builder.
    pub fn new(block_size: usize) -> Self {
        Self {
            offset: Vec::new(),
            data: Vec::new(),
            block_size,
        }
    }

    /// Adds a key-value pair to the block. Returns false when the block is full.
    #[must_use]
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> bool {
        assert!(!key.is_empty(), "key must not be empty");
        if self.current_size() + key.len() + value.len() + 3 * U16_SIZE > self.block_size
            && !self.is_empty()
        {
            return false;
        }
        self.offset.push(self.data.len() as u16);
        self.data.put_u16(key.len() as u16);
        self.data.put(key);
        self.data.put_u16(value.len() as u16);
        self.data.put(value);
        true
    }

    /// Check if there is no key-value pair in the block.s
    pub fn is_empty(&self) -> bool {
        self.offset.is_empty()
    }

    /// Finalize the block.
    pub fn build(self) -> Block {
        if self.is_empty() {
            panic!("block should not be empty");
        }
        Block {
            data: self.data,
            offsets: self.offset,
        }
    }

    fn current_size(&self) -> usize {
        self.offset.len() * U16_SIZE + self.data.len()
    }
}
