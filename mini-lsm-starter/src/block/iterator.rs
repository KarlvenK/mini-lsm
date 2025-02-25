#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use bytes::Buf;
use std::sync::Arc;

use super::Block;

/// Iterates on a block.
pub struct BlockIterator {
    block: Arc<Block>,
    key: Vec<u8>,
    value: Vec<u8>,
    idx: usize,
}

impl BlockIterator {
    fn new(block: Arc<Block>) -> Self {
        Self {
            block,
            key: Vec::new(),
            value: Vec::new(),
            idx: 0,
        }
    }

    /// Creates a block iterator and seek to the first entry.
    pub fn create_and_seek_to_first(block: Arc<Block>) -> Self {
        let mut iter = Self::new(block);
        iter.seek_to_first();
        iter
    }

    /// Creates a block iterator and seek to the first key that >= `key`.
    pub fn create_and_seek_to_key(block: Arc<Block>, key: &[u8]) -> Self {
        let mut iter = Self::new(block);
        iter.seek_to_key(key);
        iter
    }

    /// Returns the key of the current entry.
    pub fn key(&self) -> &[u8] {
        debug_assert!(!self.key.is_empty(), "invalid iterator");
        &self.key
    }

    /// Returns the value of the current entry.
    pub fn value(&self) -> &[u8] {
        debug_assert!(!self.value.is_empty(), "invalid iterator");
        &self.value
    }

    /// Returns true if the iterator is valid.
    pub fn is_valid(&self) -> bool {
        !self.key.is_empty()
    }

    /// Seeks to the first key in the block.
    pub fn seek_to_first(&mut self) {
        self.seek_to(0);
    }

    /// Move to the next key in the block.
    pub fn next(&mut self) {
        self.idx += 1;
        self.seek_to(self.idx);
    }

    /// Seek to the first key that >= `key`.
    pub fn seek_to_key(&mut self, key: &[u8]) {
        let mut left = 0;
        let mut right = self.block.offsets.len();
        while left < right {
            let mid = left + (right - left) / 2;
            self.seek_to(mid);
            assert!(self.is_valid());
            match self.key().cmp(key) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
                _ => return,
            }
        }
        self.seek_to(left);
    }

    fn seek_to(&mut self, idx: usize) {
        if idx >= self.block.offsets.len() {
            self.key.clear();
            self.value.clear();
            return;
        }
        let offset = self.block.offsets[idx] as usize;
        self.seek_to_offset(offset);
        self.idx = idx;
    }

    fn seek_to_offset(&mut self, offset: usize) {
        let mut entry = &self.block.data[offset..];
        let key_len = entry.get_u16() as usize;
        let key = entry[..key_len].to_vec();
        entry.advance(key_len);
        self.key.clear();
        self.key.extend(key);

        let value_len = entry.get_u16() as usize;
        let value = entry[..value_len].to_vec();
        entry.advance(value_len);
        self.value.clear();
        self.value.extend(value);
    }
}
