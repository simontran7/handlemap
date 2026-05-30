use super::Handle;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

pub struct DenseHandleMap<K, V> {
    data: Vec<V>,
    _marker: PhantomData<K>,
}

impl<K: Handle, V> DenseHandleMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            _marker: PhantomData,
        }
    }

    pub fn add(&mut self, element: V) -> K {
        let index = self.data.len();
        self.data.push(element);
        K::new(index)
    }

    pub fn get(&self, id: K) -> Option<&V> {
        self.data.get(id.index())
    }

    pub fn is_valid(&self, k: K) -> bool {
        k.index() < self.data.len()
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn next_key(&self) -> K {
        K::new(self.data.len())
    }

    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        (0..self.data.len()).map(K::new)
    }

    pub fn values(&self) -> slice::Iter<'_, V> {
        self.data.iter()
    }
    pub fn values_mut(&mut self) -> slice::IterMut<'_, V> {
        self.data.iter_mut()
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (K, &V)> + '_ {
        self.data.iter().enumerate().map(|(k, v)| (K::new(k), v))
    }
}

impl<K: Handle, V> Index<K> for DenseHandleMap<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &V {
        &self.data[index.index()]
    }
}
impl<K: Handle, V> IndexMut<K> for DenseHandleMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut V {
        &mut self.data[index.index()]
    }
}

impl<K: Handle, V> Default for DenseHandleMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K: Handle, V> IntoIterator for &'a DenseHandleMap<K, V> {
    type Item = &'a V;
    type IntoIter = slice::Iter<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.values()
    }
}
