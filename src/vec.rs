use core::{alloc, mem::MaybeUninit};
use std::fmt::Debug;

use crate::{const_transmute_unchecked, min};

pub struct Vec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> Debug for Vec<T, N>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        list.entries(self);
        list.finish()
    }
}

impl<'a, T: 'a, const N: usize> IntoIterator for &'a Vec<T, N> {
    type Item = &'a T;

    type IntoIter = VecIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        VecIter {
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T: 'a, const N: usize> IntoIterator for &'a mut Vec<T, N> {
    type Item = &'a mut T;

    type IntoIter = VecIterMut<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        VecIterMut {
            vec: self,
            index: 0,
        }
    }
}

pub struct VecIterMut<'a, T, const N: usize> {
    vec: &'a mut Vec<T, N>,
    index: usize,
}

pub struct VecIter<'a, T, const N: usize> {
    vec: &'a Vec<T, N>,
    index: usize,
}

impl<'a, T, const N: usize> Iterator for VecIter<'a, T, N> {
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

impl<'a, T, const N: usize> Iterator for VecIterMut<'a, T, N> {
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

impl<T, const N: usize> Default for Vec<T, N> {
    fn default() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
            len: Default::default(),
        }
    }
}

impl<T, const N: usize> Vec<T, N> {
    pub const fn from_array<const L: usize>(array: [T; L]) -> Self {
        let value: [MaybeUninit<T>; N] = unsafe { const_transmute_unchecked(array) };
        Self {
            data: value,
            len: const { min(N, L) },
        }
    }

    pub const fn from_array_and_len<const L: usize>(array: [T; L], len: usize) -> Self {
        let value: [MaybeUninit<T>; N] = unsafe { const_transmute_unchecked(array) };
        Self {
            data: unsafe { const_transmute_unchecked(value) },
            len,
        }
    }
}

impl<T, const N: usize> Vec<T, N> {
    pub const fn uninit() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; N],
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

    pub const fn as_array(&self) -> &[T; N] {
        unsafe { const_transmute_unchecked(&self.data) }
    }

    pub const fn as_array_mut(&mut self) -> &mut [T; N] {
        unsafe { const_transmute_unchecked(&mut self.data) }
    }

    pub fn iter<'a>(&'a self) -> VecIter<'a, T, N> {
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

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn clear(&mut self) {
        self.len = 0;
    }
    pub const fn capacity(&self) -> usize {
        N
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if N > self.len {
            unsafe {
                self.push_unchecked(value);
            }
            Ok(())
        } else {
            Err(value)
        }
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, value: T) {
        *self.data.get_unchecked_mut(self.len) = MaybeUninit::new(value);
        self.len = self.len.unchecked_add(1);
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.data.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.data.get_unchecked_mut(index).assume_init_mut()
    }

    #[inline(always)]
    pub unsafe fn get_transmute_unchecked<V>(&self, index: usize) -> &V {
        let value = (&self.data) as *const _ as *const T;
        &*value.offset(index as isize).cast::<V>()
    }

    #[inline(always)]
    pub unsafe fn get_transmute_mut_unchecked<V>(&mut self, index: usize) -> &mut V {
        let value = (&mut self.data) as *mut _ as *mut T;
        &mut *value.offset(index as isize).cast::<V>()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if N > index {
            Some(unsafe { self.get_unchecked(index) })
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if N > index {
            Some(unsafe { self.get_unchecked_mut(index) })
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    pub unsafe fn pop_unchecked(&mut self) -> &T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked(new_len).assume_init_ref()
    }

    pub fn pop_mut(&mut self) -> Option<&mut T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.pop_unchecked_mut() })
        }
    }

    pub unsafe fn pop_unchecked_mut(&mut self) -> &mut T {
        let new_len = self.len - 1;
        self.len = new_len;
        self.data.get_unchecked_mut(new_len).assume_init_mut()
    }
}

impl<T, const N: usize> Vec<T, N> {
    pub fn swap_remove(&mut self, index: usize) -> Result<(), ()> {
        if index < N {
            unsafe { self.swap_remove_unchecked(index) };
            Ok(())
        } else {
            Err(())
        }
    }

    pub unsafe fn swap_remove_unchecked(&mut self, index: usize) {
        let new_len = self.len - 1;
        *self.len_mut() = new_len;
        let last_ele = self.get_unchecked_mut(new_len) as *const T;
        let remove = self.get_unchecked_mut(index);
        core::ptr::copy(last_ele, remove, 1);
    }
}

impl<T: Copy, const N: usize> Clone for Vec<T, N> {
    fn clone(&self) -> Self {
        let mut vec = Self {
            data: [MaybeUninit::uninit(); N],
            len: self.len.clone(),
        };
        vec.data.copy_from_slice(self.data.as_slice());
        vec
    }
}

#[cfg(test)]
mod test {
    use crate::Vec;

    #[test]
    fn iter() {
        let mut vec = Vec::<u8, 10>::uninit();
        vec.push(1).unwrap();
        vec.push(2).unwrap();
        assert_eq!(2usize, vec.iter().count());
        assert_eq!(2u8, *vec.iter_mut().last().unwrap());
        println!("{:?}", vec);
    }

    #[test]
    fn swap_remove() {
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            vec.push(3).unwrap();
            vec.swap_remove(0).unwrap();
            assert_eq!(vec.get(0).unwrap(), &3);
            assert_eq!(vec.get(1).unwrap(), &2);
            assert_eq!(vec.len(), 2);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            vec.push(3).unwrap();
            vec.swap_remove(1).unwrap();
            assert_eq!(vec.get(0).unwrap(), &1);
            assert_eq!(vec.get(1).unwrap(), &3);
            assert_eq!(vec.len(), 2);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            vec.push(3).unwrap();
            vec.swap_remove(2).unwrap();
            assert_eq!(vec.get(0).unwrap(), &1);
            assert_eq!(vec.get(1).unwrap(), &2);
            assert_eq!(vec.len(), 2);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            vec.swap_remove(0).unwrap();
            assert_eq!(vec.get(0).unwrap(), &2);
            assert_eq!(vec.len(), 1);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            vec.swap_remove(1).unwrap();
            assert_eq!(vec.get(0).unwrap(), &1);
            assert_eq!(vec.len(), 1);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.swap_remove(0).unwrap();
            assert_eq!(vec.len(), 0);
        }
        {
            let mut vec = Vec::<u8, 10>::uninit();
            vec.push(1).unwrap();
            vec.push(2).unwrap();
            let removed = vec.swap_remove(0).unwrap();
        }
    }
}
