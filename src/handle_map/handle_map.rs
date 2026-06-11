use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

use super::Handle;

pub struct HandleMap<K, V> {
    data: Vec<V>,
    _marker: PhantomData<K>,
}

pub struct Iter<'a, K, V> {
    inner: std::iter::Enumerate<slice::Iter<'a, V>>,
    _marker: PhantomData<K>,
}

pub struct IterMut<'a, K, V> {
    inner: std::iter::Enumerate<slice::IterMut<'a, V>>,
    _marker: PhantomData<K>,
}

pub struct IntoIter<K, V> {
    inner: std::iter::Enumerate<std::vec::IntoIter<V>>,
    _marker: PhantomData<K>,
}

impl<K: Handle, V> HandleMap<K, V> {
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

    pub fn get_mut(&mut self, id: K) -> Option<&mut V> {
        self.data.get_mut(id.index())
    }

    pub fn contains_key(&self, k: K) -> bool {
        k.index() < self.data.len()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
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

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            inner: self.data.iter().enumerate(),
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            inner: self.data.iter_mut().enumerate(),
            _marker: PhantomData,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }
}

impl<K: Handle, V> Index<K> for HandleMap<K, V> {
    type Output = V;
    fn index(&self, index: K) -> &V {
        &self.data[index.index()]
    }
}

impl<K: Handle, V> IndexMut<K> for HandleMap<K, V> {
    fn index_mut(&mut self, index: K) -> &mut V {
        &mut self.data[index.index()]
    }
}

impl<K: Handle, V> Default for HandleMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Handle, V> IntoIterator for HandleMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.data.into_iter().enumerate(),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: Handle, V> IntoIterator for &'a HandleMap<K, V> {
    type Item = (K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K: Handle, V> IntoIterator for &'a mut HandleMap<K, V> {
    type Item = (K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<K: Handle, V> FromIterator<V> for HandleMap<K, V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Self {
            data: Vec::from_iter(iter),
            _marker: PhantomData,
        }
    }
}

impl<K: Handle, V> Extend<V> for HandleMap<K, V> {
    fn extend<T: IntoIterator<Item = V>>(&mut self, iter: T) {
        self.data.extend(iter);
    }
}

impl<K: Handle + fmt::Debug, V: fmt::Debug> fmt::Debug for HandleMap<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K: Handle, V> Iterator for Iter<'a, K, V> {
    type Item = (K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (K::new(k), v))
    }
}

impl<'a, K: Handle, V> Iterator for IterMut<'a, K, V> {
    type Item = (K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (K::new(k), v))
    }
}

impl<K: Handle, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (K::new(k), v))
    }
}
