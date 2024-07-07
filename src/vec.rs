use core::mem::MaybeUninit;
use std::fmt::Debug;

use generic_array::{ArrayLength, GenericArray, IntoArrayLength};
use holder::Holdable;
use mutification::ToMut;
use typenum::Const;

use crate::{
    const_transmute_unchecked, Cap, Clear, Get, GetTransmuteUnchecked, GetUnchecked, Index, Pop,
    Push,
};

#[repr(C)]
#[derive(ToMut, Holdable)]
pub struct Vec<T, N: ArrayLength> {
    data: GenericArray<MaybeUninit<T>, N>,
    len: usize,
}

impl<T, N> Debug for Vec<T, N>
where
    T: Debug,
    N: ArrayLength,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        list.entries(self);
        list.finish()
    }
}

impl<'a, T: 'a, N> IntoIterator for &'a Vec<T, N>
where
    N: ArrayLength,
{
    type Item = &'a T;

    type IntoIter = VecIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        VecIter {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T: 'a, N> IntoIterator for &'a mut Vec<T, N>
where
    N: ArrayLength,
{
    type Item = &'a mut T;

    type IntoIter = VecIterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        VecIterMut {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIterMut<'a, T, N>
where
    N: ArrayLength,
{
    vec: &'a mut Vec<T, N>,
    index: usize,
}

pub struct VecIter<'a, T, N>
where
    N: ArrayLength,
{
    vec: &'a Vec<T, N>,
    index: usize,
}

impl<'a, T, N> Iterator for VecIter<'a, T, N>
where
    N: ArrayLength,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let vec: &mut Vec<T, N> = unsafe { const_transmute_unchecked(&*self.vec) };
            let res = Some(vec.get(self.index).unwrap());
            self.index += 1;
            res
        } else {
            None
        }
    }
}

impl<'a, T, N> Iterator for VecIterMut<'a, T, N>
where
    N: ArrayLength,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let vec: &mut Vec<T, N> = unsafe { const_transmute_unchecked(&mut *self.vec) };
            let res = Some(vec.get_mut(self.index).unwrap());
            self.index += 1;
            res
        } else {
            None
        }
    }
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
    Const<{ N::USIZE }>: IntoArrayLength<ArrayLength = N>,
{
    pub const fn from_array<const L: usize>(array: [T; L]) -> Self {
        let value: [MaybeUninit<T>; N::USIZE] = unsafe { const_transmute_unchecked(array) };
        Self {
            data: GenericArray::from_array(value),
            len: L,
        }
    }

    pub const fn from_array_and_len<const L: usize>(array: [T; L], len: usize) -> Self {
        let value: [MaybeUninit<T>; N::USIZE] = unsafe { const_transmute_unchecked(array) };
        Self {
            data: unsafe { const_transmute_unchecked(value) },
            len,
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

    pub fn iter<'a>(&'a mut self) -> VecIter<'a, T, N> {
        VecIter {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> VecIterMut<'a, T, N> {
        VecIterMut {
            vec: self,
            index: 0,
        }
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
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index).assume_init_mut()
    }
}

impl<T, N> GetTransmuteUnchecked for Vec<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    unsafe fn get_transmute_unchecked<V>(&self, index: Self::Index) -> &V {
        let value = self as *const Self as *const T;
        &*value.offset(index as isize).cast::<V>()
    }

    #[inline(always)]
    unsafe fn get_transmute_mut_unchecked<V>(&mut self, index: Self::Index) -> &mut V {
        let value = self as *mut Self as *mut T;
        &mut *value.offset(index as isize).cast::<V>()
    }
}

impl<T, N> Get<T> for Vec<T, N>
where
    N: ArrayLength,
{
    fn get(&self, index: usize) -> Option<&T> {
        if N::USIZE > index {
            Some(unsafe { self.get_unchecked(index) })
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

#[cfg(test)]
mod test {
    use crate::{Push, Vec};

    #[test]
    fn iter() {
        let mut vec = Vec::<u8, typenum::U10>::uninit();
        vec.push(1);
        vec.push(2);
        assert_eq!(2usize, vec.iter().count());
        assert_eq!(2u8, *vec.iter_mut().last().unwrap());
        println!("{:?}", vec);
    }
}
