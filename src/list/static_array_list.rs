use std::mem::MaybeUninit;
use std::ptr;

pub struct StaticArrayList<E, const N: usize> {
    data: [MaybeUninit<E>; N],
    count: u32,
}

impl<E, const N: usize> StaticArrayList<E, N> {
    pub fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            count: 0,
        }
    }

    pub fn count(&self) -> usize {
        self.count as usize
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count as usize == N
    }

    pub fn add_last(&mut self, element: E) {
        assert!(!self.is_full());
        self.data[self.count as usize] = MaybeUninit::new(element);
        self.count += 1;
    }

    pub fn pop(&mut self) -> Option<E> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        Some(unsafe { self.data[self.count as usize].assume_init_read() })
    }

    pub fn get(&self, index: usize) -> &E {
        assert!(index < self.count as usize);
        unsafe { self.data[index].assume_init_ref() }
    }

    pub fn get_mut(&mut self, index: usize) -> &mut E {
        assert!(index < self.count as usize);
        unsafe { self.data[index].assume_init_mut() }
    }

    pub fn slice(&self) -> &[E] {
        unsafe {
            &*(self.data.get_unchecked(..self.count as usize) as *const [MaybeUninit<E>]
                as *const [E])
        }
    }

    pub fn slice_mut(&mut self) -> &mut [E] {
        unsafe {
            &mut *(self.data.get_unchecked_mut(..self.count as usize) as *mut [MaybeUninit<E>]
                as *mut [E])
        }
    }

    pub fn add(&mut self, index: usize, element: E) {
        assert!(!self.is_full());
        assert!(index <= self.count as usize);
        let count = self.count as usize;
        unsafe {
            ptr::copy(
                self.data.as_ptr().add(index),
                self.data.as_mut_ptr().add(index + 1),
                count - index,
            );
        }
        self.data[index] = MaybeUninit::new(element);
        self.count += 1;
    }

    pub fn remove(&mut self, index: usize) -> E {
        assert!(self.count > 0);
        assert!(index < self.count as usize);
        let result = unsafe { self.data[index].assume_init_read() };
        let count = self.count as usize;
        unsafe {
            ptr::copy(
                self.data.as_ptr().add(index + 1),
                self.data.as_mut_ptr().add(index),
                count - index - 1,
            );
        }
        self.count -= 1;
        result
    }

    pub fn truncate(&mut self, new_count: usize) {
        assert!(new_count <= self.count as usize);
        let old_count = self.count as usize;
        for i in new_count..old_count {
            unsafe { self.data[i].assume_init_drop() };
        }
        self.count = new_count as u32;
    }

    pub fn clear(&mut self) {
        self.truncate(0);
    }

    pub fn last(&self) -> Option<&E> {
        if self.count == 0 {
            return None;
        }
        Some(unsafe { self.data[self.count as usize - 1].assume_init_ref() })
    }

    pub fn last_mut(&mut self) -> Option<&mut E> {
        if self.count == 0 {
            return None;
        }
        Some(unsafe { self.data[self.count as usize - 1].assume_init_mut() })
    }

    pub fn iter(&self) -> std::slice::Iter<'_, E> {
        self.slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, E> {
        self.slice_mut().iter_mut()
    }
}

impl<T, const N: usize> Default for StaticArrayList<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Clone, const N: usize> TryFrom<&[E]> for StaticArrayList<E, N> {
    type Error = ();
    fn try_from(items: &[E]) -> Result<Self, ()> {
        if items.len() > N {
            return Err(());
        }
        let mut list = Self::new();
        for item in items {
            list.add_last(item.clone());
        }
        Ok(list)
    }
}

impl<T, const N: usize> Drop for StaticArrayList<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const CAPACITY: usize = 8;
    const NUM_OPS: usize = 12;

    fn pick_op(weights: &[u32; NUM_OPS], total: u32, roll: u32) -> usize {
        let mut remaining = roll % total;
        for (i, &w) in weights.iter().enumerate() {
            if remaining < w {
                return i;
            }
            remaining -= w;
        }
        unreachable!()
    }

    proptest! {
        #[test]
        fn matches_vec_model(
            raw_weights in prop::collection::vec(0u32..100, NUM_OPS..=NUM_OPS),
            steps in prop::collection::vec((any::<u32>(), any::<u8>(), 0usize..CAPACITY), 0..500),
        ) {
            let mut weights = [0u32; NUM_OPS];
            let mut any_nonzero = false;
            for (i, &w) in raw_weights.iter().enumerate() {
                weights[i] = w;
                if w > 0 { any_nonzero = true; }
            }
            if !any_nonzero { weights[0] = 1; }
            let total: u32 = weights.iter().sum();

            let mut list = StaticArrayList::<u8, CAPACITY>::new();
            let mut model = Vec::<u8>::new();

            for &(roll, value, index) in &steps {
                match pick_op(&weights, total, roll) {
                    // push
                    0 => {
                        if model.len() < CAPACITY {
                            list.add_last(value);
                            model.push(value);
                        }
                    }
                    // pop
                    1 => {
                        assert_eq!(list.pop(), model.pop());
                    }
                    // get
                    2 => {
                        if !model.is_empty() {
                            let i = index % model.len();
                            assert_eq!(*list.get(i), model[i]);
                        }
                    }
                    // insert_at
                    3 => {
                        if model.len() < CAPACITY {
                            let i = index % (model.len() + 1);
                            list.add(i, value);
                            model.insert(i, value);
                        }
                    }
                    // remove
                    4 => {
                        if !model.is_empty() {
                            let i = index % model.len();
                            assert_eq!(list.remove(i), model.remove(i));
                        }
                    }
                    // count
                    5 => {
                        assert_eq!(list.count(), model.len());
                    }
                    // is_empty
                    6 => {
                        assert_eq!(list.is_empty(), model.is_empty());
                    }
                    // is_full
                    7 => {
                        assert_eq!(list.is_full(), model.len() == CAPACITY);
                    }
                    // capacity
                    8 => {
                        assert_eq!(list.capacity(), CAPACITY);
                    }
                    // slice
                    9 => {
                        assert_eq!(list.slice(), model.as_slice());
                    }
                    // truncate
                    10 => {
                        if !model.is_empty() {
                            let new_len = index % (model.len() + 1);
                            list.truncate(new_len);
                            model.truncate(new_len);
                        }
                    }
                    // clear
                    11 => {
                        list.clear();
                        model.clear();
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
