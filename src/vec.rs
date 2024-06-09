use core::mem::MaybeUninit;

use generic_array::{ArrayLength, GenericArray};

pub struct Vec<T, N: ArrayLength> {
    data: GenericArray<MaybeUninit<T>, N>,
    len: usize,
}

impl<T, N: ArrayLength> Vec<T, N> {
    pub const fn uninit() -> Self {
        Self {
            data: GenericArray::uninit(),
            len: 0,
        }
    }

    pub unsafe fn push_unchecked(&mut self, value: T) {
        *self.data.get_unchecked_mut(self.len) = MaybeUninit::new(value);
        self.len += 1;
    }

    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if N::USIZE > self.len {
            unsafe {
                self.push_unchecked(value);
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index).assume_init_ref()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if N::USIZE > index {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index).assume_init_mut()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if N::USIZE > index {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub unsafe fn pop_unchecked(&mut self) -> &T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked(new_len).assume_init_ref()
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    pub unsafe fn pop_unchecked_mut(&mut self) -> &mut T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked_mut(new_len).assume_init_mut()
    }

    pub fn pop_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked_mut() })
        }
    }
}
