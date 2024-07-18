use std::marker::PhantomData;

use crate::{Clear, Get, GetUnchecked, Index, Pop, Push, RemoveUnchecked, Vec};

///Simply store element fast without any other features like get length, and iteration.
pub struct Slab<I, T, const N: usize> {
    chunk: Vec<T, N>,
    spares: Vec<usize, N>,
    _marker: PhantomData<I>,
}

pub auto trait NotCloneAndCopy {}
impl<T: Clone + Copy> !NotCloneAndCopy for T {}

impl<'a, I, T: 'a, const N: usize> Get<'a, T> for Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
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
            _marker: PhantomData,
        }
    }
}

impl<'a, I, T, const N: usize> Index<'a> for Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
{
    type Index = &'a I;
}

impl<'a, I, T: 'a, const N: usize> GetUnchecked<'a, T> for Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
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

impl<'a, I, T: 'a, const N: usize> RemoveUnchecked<'a> for Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
{
    unsafe fn remove_unchecked(&mut self, index: Self::Index) {
        self.spares.push_unchecked(index.into());
    }
}

impl<'a, I, T, const N: usize> AddWithIndex<T> for Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
{
    #[inline(always)]
    fn add_with_index<F>(&mut self, f: F) -> Result<usize, ()>
    where
        F: FnOnce(&usize) -> T,
    {
        if self.spares.len() == 0 {
            let index = self.chunk.len();
            if index == N {
                return Err(());
            }
            let elem = f(&index);
            unsafe { self.chunk.push_unchecked(elem) };
            Ok(index)
        } else {
            let index = unsafe { self.spares.pop_unchecked() };
            let elem = f(index);
            *unsafe { self.chunk.get_unchecked_mut(*index) } = elem;
            Ok(*index)
        }
    }
}

impl<'a, I, T, const N: usize> Slab<I, T, N>
where
    &'a I: Into<usize> + 'a,
{
    pub fn new() -> Slab<I, T, N> {
        Slab {
            chunk: Vec::uninit(),
            spares: Vec::uninit(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{AddWithIndex, Get, GetUnchecked, NotCloneAndCopy};

    use super::Slab;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq, Eq)]
        struct A {
            inner: usize,
            index: usize,
        }
        struct Id(usize);
        impl NotCloneAndCopy for Id {}
        impl Into<usize> for &Id {
            fn into(self) -> usize {
                self.0
            }
        }
        let mut value = Slab::<Id, A, 100>::new();
        value
            .add_with_index(|index| A {
                inner: 123,
                index: *index,
            })
            .unwrap();
        value
            .add_with_index(|index| A {
                inner: 456,
                index: *index,
            })
            .unwrap();
        assert_eq!(
            unsafe { value.get_unchecked(&Id(0)) },
            &A {
                inner: 123,
                index: 0
            }
        );
        assert_eq!(
            unsafe { value.get_unchecked(&Id(1)) },
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
        impl Into<usize> for &Id {
            fn into(self) -> usize {
                self.0
            }
        }
        let slab: Slab<Id, bool, 100> = Slab::new();
        let value = slab.get(&Id(0));
    }
}
