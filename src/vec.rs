use core::mem::MaybeUninit;

use generic_array::{ArrayLength, GenericArray, IntoArrayLength};
use typenum::Const;

use crate::{const_transmute_unchecked, Cap, Clear, Get, GetUnchecked, Index, Pop, Push};

#[derive(Debug)]
pub struct Vec<T, N: ArrayLength> {
    data: GenericArray<MaybeUninit<T>, N>,
    len: usize,
}

impl<T, N> Default for Vec<T, N>
where
    N: ArrayLength,
{
    fn default() -> Self {
        Self {
            data: GenericArray::uninit(),
            len: Default::default(),
        }
    }
}

impl<T, N> Vec<T, N>
where
    N: ArrayLength,
{
    pub const fn uninit() -> Self {
        Self {
            data: GenericArray::uninit(),
            len: 0,
        }
    }

    pub const fn from_array<const L: usize>(array: [T; L]) -> Self
    where
        Const<L>: IntoArrayLength<ArrayLength = N>,
    {
        Self {
            data: unsafe { const_transmute_unchecked(array) },
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const unsafe fn len_mut(&mut self) -> &mut usize {
        &mut self.len
    }

    pub const fn as_slice(&self) -> &[T] {
        unsafe { const_transmute_unchecked(self.data.as_slice()) }
    }

    pub const fn as_array(&self) -> &[T; N::USIZE] {
        unsafe { const_transmute_unchecked(&self.data) }
    }

    pub const fn as_array_mut(&mut self) -> &mut [T; N::USIZE] {
        unsafe { const_transmute_unchecked(&mut self.data) }
    }
}

impl<T, N> Clear for Vec<T, N>
where
    N: ArrayLength,
{
    fn clear(&mut self) {
        self.len = 0;
    }
}

impl<T, N> Index for Vec<T, N>
where
    N: ArrayLength,
{
    type Index = usize;
}

impl<T, N> Cap for Vec<T, N>
where
    N: ArrayLength,
{
    type Cap = usize;

    fn capacity(&self) -> Self::Cap {
        N::USIZE
    }
}

impl<T, N> Push<T> for Vec<T, N>
where
    N: ArrayLength,
{
    fn push(&mut self, value: T) -> Result<(), T> {
        if N::USIZE > self.len {
            unsafe {
                self.push_unchecked(value);
            }
            Ok(())
        } else {
            Err(value)
        }
    }

    #[inline(always)]
    unsafe fn push_unchecked(&mut self, value: T) {
        *self.data.get_unchecked_mut(self.len) = MaybeUninit::new(value);
        self.len = self.len.unchecked_add(1);
    }
}

impl<T, N> GetUnchecked<T> for Vec<T, N>
where
    N: ArrayLength,
{
    unsafe fn get_unchecked_ref(&self, index: usize) -> &T {
        self.data.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index).assume_init_mut()
    }
}

impl<T, N> Get<T> for Vec<T, N>
where
    N: ArrayLength,
{
    fn get(&self, index: usize) -> Option<&T> {
        if N::USIZE > index {
            Some(unsafe { self.get_unchecked_ref(index) })
        } else {
            None
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if N::USIZE > index {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }
}

impl<T, N> Pop<T> for Vec<T, N>
where
    N: ArrayLength,
{
    fn pop(&mut self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    unsafe fn pop_unchecked(&mut self) -> &T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked(new_len).assume_init_ref()
    }

    fn pop_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked_mut() })
        }
    }

    unsafe fn pop_unchecked_mut(&mut self) -> &mut T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked_mut(new_len).assume_init_mut()
    }
}
