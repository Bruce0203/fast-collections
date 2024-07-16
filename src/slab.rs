use std::marker::PhantomData;

use crate::{Clear, Get, GetUnchecked, Index, Pop, Push, RemoveUnchecked, Vec};

///Simply store element fast without any other features like get length, and iteration.
pub struct Slab<I, T, const N: usize> {
    chunk: Vec<T, N>,
    spares: Vec<usize, N>,
    item_ptrs: Vec<usize, N>,
    _marker: PhantomData<I>,
}

pub auto trait NotCloneAndCopy {}
impl<T: Clone + Copy> !NotCloneAndCopy for T {}

impl<I, T, const N: usize> Get<T> for Slab<I, T, N>
where
    I: Into<usize> + NotCloneAndCopy,
{
    fn get(&self, index: Self::Index) -> Option<&T> {
        Some(unsafe { self.get_unchecked(index) })
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut T> {
        Some(unsafe { self.get_unchecked_mut(index) })
    }
}

impl<I, T, const N: usize> Default for Slab<I, T, N> {
    fn default() -> Self {
        Self {
            chunk: Default::default(),
            spares: Default::default(),
            item_ptrs: Default::default(),
            _marker: PhantomData,
        }
    }
}

impl<I, T, const N: usize> Index for Slab<I, T, N>
where
    I: Into<usize>,
{
    type Index = I;
}

impl<I, T, const N: usize> GetUnchecked<T> for Slab<I, T, N>
where
    I: Into<usize>,
{
    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    unsafe fn get_unchecked_mut(&mut self, index: Self::Index) -> &mut T {
        self.chunk.get_unchecked_mut(index.into())
    }

    unsafe fn get_unchecked(&self, index: Self::Index) -> &T {
        self.chunk.get_unchecked(index.into())
    }
}

impl<I, T, const N: usize> Clear for Slab<I, T, N> {
    fn clear(&mut self) {
        self.chunk.clear();
        self.spares.clear();
    }
}

impl<I, T, const N: usize> RemoveUnchecked for Slab<I, T, N>
where
    I: Into<usize>,
{
    unsafe fn remove_unchecked(&mut self, index: Self::Index) {
        self.spares.push_unchecked(index.into());
    }
}

impl<I, T, const N: usize> Slab<I, T, N>
where
    I: Into<usize>,
{
    pub fn new() -> Slab<I, T, N> {
        Slab {
            chunk: Vec::uninit(),
            spares: Vec::uninit(),
            _marker: PhantomData,
            item_ptrs: Vec::uninit(),
        }
    }

    #[inline(always)]
    pub fn add_with_index<F>(&mut self, f: F) -> Result<usize, ()>
    where
        F: FnOnce(usize) -> T,
    {
        if self.spares.len() == 0 {
            let index = self.chunk.len();
            if index == N {
                return Err(());
            }
            let elem = f(index);
            unsafe { self.chunk.push_unchecked(elem) };
            Ok(index)
        } else {
            let index = *unsafe { self.spares.pop_unchecked() };
            let elem = f(index);
            *unsafe { self.chunk.get_unchecked_mut(index) } = elem;
            Ok(index)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Get, GetUnchecked, NotCloneAndCopy};

    use super::Slab;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq, Eq)]
        struct A {
            inner: usize,
            index: usize,
        }
        let mut value = Slab::<usize, A, 100>::new();
        value
            .add_with_index(|index| A { inner: 123, index })
            .unwrap();
        value
            .add_with_index(|index| A { inner: 456, index })
            .unwrap();
        assert_eq!(
            unsafe { value.get_unchecked(0) },
            &A {
                inner: 123,
                index: 0
            }
        );
        assert_eq!(
            unsafe { value.get_unchecked(1) },
            &A {
                inner: 456,
                index: 1
            }
        );
    }

    #[test]
    fn not_clone_copy_test() {
        pub struct Id(usize);
        impl NotCloneAndCopy for Id {}
        impl Into<usize> for Id {
            fn into(self) -> usize {
                self.0
            }
        }
        let slab: Slab<Id, bool, 100> = Slab::new();
        let value = slab.get(Id(0));
    }
}
