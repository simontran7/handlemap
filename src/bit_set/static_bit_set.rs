use crate::bit_set::word::Word;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticBitSet<W: Word, const WORDS: usize> {
    data: [W; WORDS],
}

pub struct StaticBitSetIterator<'a, W: Word, const WORDS: usize> {
    bitset: &'a StaticBitSet<W, WORDS>,
    prev: Option<usize>,
}

impl<W: Word, const WORDS: usize> StaticBitSet<W, WORDS> {
    /// Creates a new empty bit set.
    pub const fn new() -> Self {
        Self {
            data: [W::ZERO; WORDS],
        }
    }

    /// Creates a bit set from a slice `bits`
    pub fn from<const N: usize>(bits: &[usize; N]) -> Self {
        let mut data = [W::ZERO; WORDS];
        let mut i = 0;
        while i < N {
            let bit = bits[i];
            let word_idx = bit / W::BITS;
            let bit_offset = (bit % W::BITS) as u32;
            data[word_idx] = data[word_idx] | (W::ONE << bit_offset);
            i += 1;
        }
        Self { data }
    }

    /// Returns whether `bit` is in the set.
    pub fn contains(&self, bit: usize) -> bool {
        let word_idx = bit / W::BITS;
        let bit_offset = (bit % W::BITS) as u32;
        (self.data[word_idx] & (W::ONE << bit_offset)) != W::ZERO
    }

    /// Adds `bit` to the set.
    pub fn add(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_offset = (bit % W::BITS) as u32;
        self.data[word_idx] |= W::ONE << bit_offset;
    }

    /// Removes `bit` from the set.
    pub fn remove(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_offset = (bit % W::BITS) as u32;
        self.data[word_idx] &= !(W::ONE << bit_offset);
    }

    /// Toggles `bit`.
    pub fn toggle(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_offset = (bit % W::BITS) as u32;
        self.data[word_idx] ^= W::ONE << bit_offset;
    }

    /// Clears the set, removing all bits.
    pub fn clear(&mut self) {
        for w in self.data.iter_mut() {
            *w = W::ZERO;
        }
    }

    /// Fills the set, adding all possible bits.
    pub fn fill(&mut self) {
        for w in self.data.iter_mut() {
            *w = W::ONES;
        }
    }

    /// Returns the number of bits set.
    pub fn count(&self) -> usize {
        self.data.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|&w| w == W::ZERO)
    }

    /// Returns whether the set is full.
    pub fn is_full(&self) -> bool {
        self.data.iter().all(|&w| w == W::ONES)
    }

    /// Returns the smallest bit in the set, or `None` if empty.
    pub fn first_set(&self) -> Option<usize> {
        for (word_idx, word) in self.data.iter().enumerate() {
            let tz = word.trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_idx + tz as usize);
            }
        }
