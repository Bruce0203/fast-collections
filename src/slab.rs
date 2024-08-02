use std::ptr::drop_in_place;

use crate::Vec;

///Fast and simple storage without any other features like get length, and iteration.
pub struct Slab<T, const N: usize> {
    chunk: Vec<T, N>,
    spares: Vec<usize, N>,
}

impl<T, const N: usize> Slab<T, N> {
    pub fn new() -> Self {
        Self {
            chunk: Vec::uninit(),
            spares: Vec::uninit(),
        }
    }

    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.chunk.get_unchecked_mut(index.into())
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.chunk.get_unchecked(index.into())
    }

    pub fn clear(&mut self) {
        for ele in self.chunk.iter_mut() {
            unsafe { drop_in_place(ele as *mut T) }
        }
        self.chunk.clear();
        self.spares.clear();
    }

    pub unsafe fn remove_unchecked(&mut self, index: usize) {
        core::ptr::drop_in_place(self.chunk.get_unchecked_mut(index.into()));
        self.spares.push_unchecked(index.into());
    }

    #[inline(always)]
    pub fn add_with_index<F>(&mut self, f: F) -> Result<usize, ()>
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

impl<T, const N: usize> Default for Slab<T, N> {
    fn default() -> Self {
        Self {
            chunk: Default::default(),
            spares: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Slab;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq, Eq)]
        struct A {
            inner: usize,
            index: usize,
        }
        struct Id(usize);
        impl Into<usize> for &Id {
            fn into(self) -> usize {
                self.0
            }
        }
        let mut value = Slab::<A, 100>::new();
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
    fn drop_test() {
        struct Token(usize);
        let mut slab: Slab<Token, 10> = Slab::new();
        static mut value: bool = false;
        impl Drop for Token {
            fn drop(&mut self) {
                self.0 = 123;
                unsafe { value = true };
                println!("HI");
            }
        }
        slab.add_with_index(|i| Token(1)).unwrap();
        unsafe { slab.remove_unchecked(0) };
        assert_eq!(unsafe { value }, true)
    }
}
