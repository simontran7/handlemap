use std::marker::PhantomData;
use std::ops::{Index, IndexMut};
use std::slice;

pub struct HandleMap<I, T> {
    data: Vec<T>,
    _marker: PhantomData<I>,
}

impl<I, T> HandleMap<I, T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn add(&mut self, element: T) -> I
    where
        I: From<usize>,
    {
        let index = self.data.len();
        self.data.push(element);
        I::from(index)
    }

    pub fn get(&self, index: I) -> Option<&T>
    where
        I: Into<usize>,
    {
        self.data.get(index.into())
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.data.iter_mut()
    }

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (I, &T)> + '_
    where
        I: From<usize>,
    {
        self.data.iter().enumerate().map(|(i, t)| (I::from(i), t))
    }
}

impl<I: Into<usize>, T> Index<I> for HandleMap<I, T> {
    type Output = T;
    fn index(&self, index: I) -> &T {
        &self.data[index.into()]
    }
}

impl<I: Into<usize>, T> IndexMut<I> for HandleMap<I, T> {
    fn index_mut(&mut self, index: I) -> &mut T {
        &mut self.data[index.into()]
    }
}

impl<'a, I, T> IntoIterator for &'a HandleMap<I, T> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
