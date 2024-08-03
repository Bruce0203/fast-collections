use std::{mem::MaybeUninit, ptr::drop_in_place};

use crate::const_transmute_unchecked;

///Fast and simple storage without any other features like get length, and iteration.
pub struct Slab<T, const N: usize> {
    chunk: [MaybeUninit<T>; N],
    spares: [MaybeUninit<usize>; N],
    chunk_len: usize,
    spares_len: usize,
}

impl<T, const N: usize> Slab<T, N> {
    pub fn new() -> Self {
        Self {
            chunk: unsafe { const_transmute_unchecked(()) },
            spares: unsafe { const_transmute_unchecked(()) },
            chunk_len: 0,
            spares_len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.chunk_len - self.spares_len
    }

    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.chunk.get_unchecked_mut(index).assume_init_mut()
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.chunk.get_unchecked(index).assume_init_ref()
    }

    pub fn clear(&mut self) {
        for ele in self.chunk.iter_mut() {
            unsafe { drop_in_place(ele.assume_init_mut() as *mut T) }
        }
        self.chunk_len = 0;
        self.spares_len = 0
    }

    pub unsafe fn remove_unchecked(&mut self, index: usize) {
        core::ptr::drop_in_place(self.chunk.get_unchecked_mut(index).assume_init_mut());
        {
            let spares_len = self.spares_len;
            *self.spares.get_unchecked_mut(spares_len) = MaybeUninit::new(index);
            self.spares_len = spares_len + 1;
        }
    }

    #[inline(always)]
    pub fn add_with_index<F>(&mut self, f: F) -> Result<usize, ()>
    where
        F: FnOnce(&usize) -> T,
    {
        let spares_len = self.spares_len;
        if spares_len == 0 {
            let chunk_len = self.chunk_len;
            let index = chunk_len;
            if index == N {
                return Err(());
            }
            let elem = f(&index);
            unsafe {
                *self.chunk.get_unchecked_mut(chunk_len) = MaybeUninit::new(elem);
                self.chunk_len = chunk_len + 1;
            };
            Ok(index)
        } else {
            let index = unsafe {
                let new_spares_len = spares_len - 1;
                self.spares_len = new_spares_len;
                self.spares
                    .get_unchecked_mut(new_spares_len)
                    .assume_init_ref()
            };
            let elem = f(index);
            *unsafe { self.chunk.get_unchecked_mut(*index) } = MaybeUninit::new(elem);
            Ok(*index)
        }
    }
}

impl<T, const N: usize> Default for Slab<T, N> {
    fn default() -> Self {
        Self::new()
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
        static mut VALUE: bool = false;
        impl Drop for Token {
            fn drop(&mut self) {
                self.0 = 123;
                unsafe { VALUE = true };
                println!("HI");
            }
        }
        slab.add_with_index(|_i| Token(1)).unwrap();
        unsafe { slab.remove_unchecked(0) };
        assert_eq!(unsafe { VALUE }, true)
    }

    #[test]
    fn simple_test() {
        let mut slab: Slab<u8, 10> = Slab::new();
        let index = slab
            .add_with_index(|index| {
                assert_eq!(index, &0);
                123
            })
            .unwrap();
        assert_eq!(index, 0);
        let value = *unsafe { slab.get_unchecked_mut(index) };
        assert_eq!(value, 123);
        let id2 = slab
            .add_with_index(|index| {
                assert_eq!(index, &1);
                222
            })
            .unwrap();
        assert_eq!(id2, 1);
    }
}
