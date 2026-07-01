use crate::bit_set::word::Word;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StaticBitSet<const WORDS: usize, W: Word = u64> {
    words: [W; WORDS],
}

pub struct StaticBitSetIterator<'a, const WORDS: usize, W: Word> {
    bitset: &'a StaticBitSet<WORDS, W>,
    last_yielded: Option<usize>,
}

impl<const WORDS: usize, W: Word> StaticBitSet<WORDS, W> {
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
    pub fn count(&self) -> usize {
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
    pub fn iter(&self) -> StaticBitSetIterator<'_, WORDS, W> {
        StaticBitSetIterator {
            bitset: self,
            last_yielded: None,
        }
    }
}

impl<'a, const WORDS: usize, W: Word> Iterator for StaticBitSetIterator<'a, WORDS, W> {
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
impl<'a, const WORDS: usize, W: Word> IntoIterator for &'a StaticBitSet<WORDS, W> {
    type Item = usize;
    type IntoIter = StaticBitSetIterator<'a, WORDS, W>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<const WORDS: usize, W: Word> Default for StaticBitSet<WORDS, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const WORDS: usize, W: Word> BitOrAssign<&Self> for StaticBitSet<WORDS, W> {
    fn bitor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] |= rhs.words[i];
        }
    }
}
impl<const WORDS: usize, W: Word> BitAndAssign<&Self> for StaticBitSet<WORDS, W> {
    fn bitand_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] &= rhs.words[i];
        }
    }
}
impl<const WORDS: usize, W: Word> BitXorAssign<&Self> for StaticBitSet<WORDS, W> {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for i in 0..WORDS {
            self.words[i] ^= rhs.words[i];
        }
    }
}

impl<const WORDS: usize, W: Word> BitOr for &StaticBitSet<WORDS, W> {
    type Output = StaticBitSet<WORDS, W>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] | rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> BitAnd for &StaticBitSet<WORDS, W> {
    type Output = StaticBitSet<WORDS, W>;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] & rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> BitXor for &StaticBitSet<WORDS, W> {
    type Output = StaticBitSet<WORDS, W>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.words[i] = self.words[i] ^ rhs.words[i];
        }
        result
    }
}
impl<const WORDS: usize, W: Word> Not for &StaticBitSet<WORDS, W> {
    type Output = StaticBitSet<WORDS, W>;
    fn not(self) -> Self::Output {
        let mut result = StaticBitSet::new();
        for i in 0..WORDS {
            result.words[i] = !self.words[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use std::collections::BTreeSet;

    const CAPACITY: usize = 256;

    #[derive(Debug, Clone)]
    enum Op {
        Insert(usize),
        Remove(usize),
        Toggle(usize),
        Contains(usize),
        Count,
        IsEmpty,
        IsFull,
        FirstSet,
        LastSet,
        FirstUnset,
        LastUnset,
        FirstSetAfter(usize),
        LastSetBefore(usize),
        FirstUnsetAfter(usize),
        LastUnsetBefore(usize),
        Clear,
        Iter,
    }

    const NUM_OPS: usize = 17;

    fn pick_op(weights: &[u32; NUM_OPS], total: u32, roll: u32, bit: usize) -> Op {
        let mut remaining = roll % total;
        for (i, &w) in weights.iter().enumerate() {
            if remaining < w {
                return match i {
                    0 => Op::Insert(bit),
                    1 => Op::Remove(bit),
                    2 => Op::Toggle(bit),
                    3 => Op::Contains(bit),
                    4 => Op::Count,
                    5 => Op::IsEmpty,
                    6 => Op::IsFull,
                    7 => Op::FirstSet,
                    8 => Op::LastSet,
                    9 => Op::FirstUnset,
                    10 => Op::LastUnset,
                    11 => Op::FirstSetAfter(bit),
                    12 => Op::LastSetBefore(bit),
                    13 => Op::FirstUnsetAfter(bit),
                    14 => Op::LastUnsetBefore(bit),
                    15 => Op::Clear,
                    16 => Op::Iter,
                    _ => unreachable!(),
                };
            }
            remaining -= w;
        }
        unreachable!()
    }

    proptest! {
        #[test]
        fn matches_btreeset_model(
            raw_weights in prop::collection::vec(0u32..100, NUM_OPS..=NUM_OPS),
            steps in prop::collection::vec((any::<u32>(), 0usize..CAPACITY), 0..500),
        ) {
            let mut weights = [0u32; NUM_OPS];
            let mut any_nonzero = false;
            for (i, &w) in raw_weights.iter().enumerate() {
                weights[i] = w;
                if w > 0 { any_nonzero = true; }
            }
            if !any_nonzero { weights[0] = 1; }
            let total: u32 = weights.iter().sum();

            let mut bs = StaticBitSet::<4, u64>::new();
            let mut model = BTreeSet::<usize>::new();

            for &(roll, bit) in &steps {
                let op = pick_op(&weights, total, roll, bit);
                match op {
                    Op::Insert(i) => {
                        bs.insert(i);
                        model.insert(i);
                    }
                    Op::Remove(i) => {
                        bs.remove(i);
                        model.remove(&i);
                    }
                    Op::Toggle(i) => {
                        bs.toggle(i);
                        if !model.remove(&i) {
                            model.insert(i);
                        }
                    }
                    Op::Contains(i) => {
                        assert_eq!(bs.contains(i), model.contains(&i));
                    }
                    Op::Count => {
                        assert_eq!(bs.count(), model.len());
                    }
                    Op::IsEmpty => {
                        assert_eq!(bs.is_empty(), model.is_empty());
                    }
                    Op::IsFull => {
                        assert_eq!(bs.is_full(), model.len() == CAPACITY);
                    }
                    Op::FirstSet => {
                        assert_eq!(bs.first_set(), model.first().copied());
                    }
                    Op::LastSet => {
                        assert_eq!(bs.last_set(), model.last().copied());
                    }
                    Op::FirstUnset => {
                        let expected = (0..CAPACITY).find(|i| !model.contains(i));
                        assert_eq!(bs.first_unset(), expected);
                    }
                    Op::LastUnset => {
                        let expected = (0..CAPACITY).rev().find(|i| !model.contains(i));
                        assert_eq!(bs.last_unset(), expected);
                    }
                    Op::FirstSetAfter(i) => {
                        let expected = ((i + 1)..CAPACITY).find(|j| model.contains(j));
                        assert_eq!(bs.first_set_after(i), expected);
                    }
                    Op::LastSetBefore(i) => {
                        let expected = (0..i).rev().find(|j| model.contains(j));
                        assert_eq!(bs.last_set_before(i), expected);
                    }
                    Op::FirstUnsetAfter(i) => {
                        let expected = ((i + 1)..CAPACITY).find(|j| !model.contains(j));
                        assert_eq!(bs.first_unset_after(i), expected);
                    }
                    Op::LastUnsetBefore(i) => {
                        let expected = (0..i).rev().find(|j| !model.contains(j));
                        assert_eq!(bs.last_unset_before(i), expected);
                    }
                    Op::Clear => {
                        bs.clear();
                        model.clear();
                    }
                    Op::Iter => {
                        let bs_bits: Vec<usize> = bs.iter().collect();
                        let model_bits: Vec<usize> = model.iter().copied().collect();
                        assert_eq!(bs_bits, model_bits);
                    }
                }
            }
        }
    }
}
