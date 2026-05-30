use crate::bit_sets::word::Word;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, Shr};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlineBitSet<const WORDS: usize, W: Word = u64> {
    words: [W; WORDS],
}

pub struct InlineBitSetIterator<'a, const WORDS: usize, W: Word> {
    bitset: &'a InlineBitSet<WORDS, W>,
    last_yielded: Option<usize>,
}

impl<const WORDS: usize, W: Word> InlineBitSet<WORDS, W> {
    /// Creates a new empty bit set.
    pub fn new() -> Self {
        Self {
            words: [W::ZERO; WORDS],
        }
    }

    /// Returns whether `bit` is in the set.
    pub fn contains(&self, bit: usize) -> bool {
        let word_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;
        (self.words[word_idx] & (W::ONE << bit_pos)) != W::ZERO
    }

    /// Adds `bit` to the set.
    pub fn insert(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;
        self.words[word_idx] |= W::ONE << bit_pos;
    }

    /// Removes `bit` from the set.
    pub fn remove(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;
        self.words[word_idx] &= !(W::ONE << bit_pos);
    }

    /// Toggles `bit`.
    pub fn toggle(&mut self, bit: usize) {
        let word_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;
        self.words[word_idx] ^= W::ONE << bit_pos;
    }

    /// Clears the set, removing all bits.
    pub fn clear(&mut self) {
        for w in self.words.iter_mut() {
            *w = W::ZERO;
        }
    }

    /// Fills the set, adding all possible bits.
    pub fn fill(&mut self) {
        for w in self.words.iter_mut() {
            *w = W::ONES;
        }
    }

    /// Returns the number of bits set.
    pub fn len(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Returns whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.words.iter().all(|&w| w == W::ZERO)
    }

    /// Returns whether the set is full.
    pub fn is_full(&self) -> bool {
        self.words.iter().all(|&w| w == W::ONES)
    }

    /// Returns the smallest bit in the set, or `None` if empty.
    pub fn first_set(&self) -> Option<usize> {
        for (word_idx, word) in self.words.iter().enumerate() {
            let tz = word.trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_idx + tz as usize);
            }
        }
        None
    }

    /// Returns the largest bit in the set, or `None` if empty.
    pub fn last_set(&self) -> Option<usize> {
        for (word_idx, word) in self.words.iter().enumerate().rev() {
            let lz = word.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_idx + local_pos);
            }
        }
        None
    }

    /// Returns the smallest bit strictly greater than `bit`, or `None`.
    pub fn first_set_after(&self, bit: usize) -> Option<usize> {
        let boundary_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;

        if boundary_idx >= WORDS {
            return None;
        }

        let mask = W::ONES.checked_shl(bit_pos + 1).unwrap_or(W::ZERO);
        let masked = self.words[boundary_idx] & mask;
        let tz = masked.trailing_zeros();
        if (tz as usize) < W::BITS {
            return Some(W::BITS * boundary_idx + tz as usize);
        }

        for word_idx in (boundary_idx + 1)..WORDS {
            let tz = self.words[word_idx].trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_idx + tz as usize);
            }
        }

        None
    }

    /// Returns the largest bit strictly less than `bit`, or `None`.
    pub fn last_set_before(&self, bit: usize) -> Option<usize> {
        let boundary_idx = (bit / W::BITS).min(WORDS);
        let bit_pos = (bit % W::BITS) as u32;

        if boundary_idx < WORDS {
            let mask = W::ONES
                .checked_shr(W::BITS as u32 - bit_pos)
                .unwrap_or(W::ZERO);
            let masked = self.words[boundary_idx] & mask;
            let lz = masked.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * boundary_idx + local_pos);
            }
        }

        for word_idx in (0..boundary_idx).rev() {
            let lz = self.words[word_idx].leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_idx + local_pos);
            }
        }

        None
    }

    /// Returns the smallest bit *not* in the set, or `None` if full.
    pub fn first_unset(&self) -> Option<usize> {
        for (word_idx, word) in self.words.iter().enumerate() {
            let tz = (!*word).trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_idx + tz as usize);
            }
        }
        None
    }

    /// Returns the largest bit *not* in the set, or `None` if full.
    pub fn last_unset(&self) -> Option<usize> {
        for (word_idx, word) in self.words.iter().enumerate().rev() {
            let lz = (!*word).leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_idx + local_pos);
            }
        }
        None
    }

    /// Returns the smallest bit *not* in the set, strictly greater than `bit`.
    pub fn first_unset_after(&self, bit: usize) -> Option<usize> {
        let boundary_idx = bit / W::BITS;
        let bit_pos = (bit % W::BITS) as u32;

        if boundary_idx >= WORDS {
            return None;
        }

        let mask = W::ONES.checked_shl(bit_pos + 1).unwrap_or(W::ZERO);
        let masked = (!self.words[boundary_idx]) & mask;
        let tz = masked.trailing_zeros();
        if (tz as usize) < W::BITS {
            return Some(W::BITS * boundary_idx + tz as usize);
        }

        for word_idx in (boundary_idx + 1)..WORDS {
            let tz = (!self.words[word_idx]).trailing_zeros();
            if (tz as usize) < W::BITS {
                return Some(W::BITS * word_idx + tz as usize);
            }
        }

        None
    }

    /// Returns the largest bit *not* in the set, strictly less than `bit`.
    pub fn last_unset_before(&self, bit: usize) -> Option<usize> {
        let boundary_idx = (bit / W::BITS).min(WORDS);
        let bit_pos = (bit % W::BITS) as u32;

        if boundary_idx < WORDS {
            let mask = W::ONES
                .checked_shr(W::BITS as u32 - bit_pos)
                .unwrap_or(W::ZERO);
            let masked = (!self.words[boundary_idx]) & mask;
            let lz = masked.leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * boundary_idx + local_pos);
            }
        }

        for word_idx in (0..boundary_idx).rev() {
            let lz = (!self.words[word_idx]).leading_zeros();
            if (lz as usize) < W::BITS {
                let local_pos = W::BITS - 1 - lz as usize;
                return Some(W::BITS * word_idx + local_pos);
            }
        }

        None
    }

    /// Returns a read-only view of the underlying words.
    pub fn words(&self) -> &[W] {
        &self.words
    }

    /// Returns a mutable view of the underlying words.
    pub fn words_mut(&mut self) -> &mut [W] {
        &mut self.words
    }

    /// Returns an iterator over the set bits, in ascending order.
    pub fn iter(&self) -> InlineBitSetIterator<'_, WORDS, W> {
        InlineBitSetIterator {
            bitset: self,
            last_yielded: None,
        }
    }
}

impl<'a, const WORDS: usize, W: Word> Iterator for InlineBitSetIterator<'a, WORDS, W> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let next_bit = match self.last_yielded {
            None => self.bitset.first_set(),
            Some(prev) => self.bitset.first_set_after(prev),
        };
        self.last_yielded = next_bit;
        next_bit
    }
}
impl<'a, const WORDS: usize, W: Word> IntoIterator for &'a InlineBitSet<WORDS, W> {
    type Item = usize;
    type IntoIter = InlineBitSetIterator<'a, WORDS, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const WORDS: usize, W: Word> Default for InlineBitSet<WORDS, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WORDS: usize, W: Word> BitOrAssign<&Self> for InlineBitSet<WORDS, W> {
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] |= rhs.words[i];
        }
    }
}
impl<const WORDS: usize, W: Word> BitAndAssign<&Self> for InlineBitSet<WORDS, W> {
    fn bitand_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] &= rhs.words[i];
        }
    }
}
impl<const WORDS: usize, W: Word> BitXorAssign<&Self> for InlineBitSet<WORDS, W> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] ^= rhs.words[i];
        }
    }
}

impl<const WORDS: usize, W: Word> BitOr for &InlineBitSet<WORDS, W> {
    type Output = InlineBitSet<WORDS, W>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = InlineBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] | rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> BitAnd for &InlineBitSet<WORDS, W> {
    type Output = InlineBitSet<WORDS, W>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = InlineBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] & rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> BitXor for &InlineBitSet<WORDS, W> {
    type Output = InlineBitSet<WORDS, W>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = InlineBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] ^ rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> Not for &InlineBitSet<WORDS, W> {
    type Output = InlineBitSet<WORDS, W>;
    fn not(self) -> Self::Output {
        let mut result = InlineBitSet::new();
        for i in 0..WORDS {
            result.words[i] = !self.words[i];
        }
        result
    }
}
