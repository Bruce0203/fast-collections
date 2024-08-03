use std::mem::MaybeUninit;

use crate::{const_transmute_unchecked, Slab};

pub struct IterableSlab<T, const N: usize> {
    slab: Slab<T, N>,
    ids: [MaybeUninit<usize>; N],
}

impl<T, const N: usize> Default for IterableSlab<T, N> {
    fn default() -> Self {
        Self {
            slab: Default::default(),
            ids: unsafe { const_transmute_unchecked(()) },
        }
    }
}

impl<T, const N: usize> IterableSlab<T, N> {
    pub fn new() -> Self {
        Default::default()
    }

    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.slab.get_unchecked_mut(index)
    }

    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        self.slab.get_unchecked(index)
    }

    pub fn clear(&mut self) {
        self.slab.clear()
    }

    pub unsafe fn remove_unchecked(&mut self, index: usize) {
        self.slab.remove_unchecked(index)
    }

    #[inline(always)]
    pub fn add_with_index<F>(&mut self, f: F) -> Result<usize, ()>
    where
        F: FnOnce(&usize) -> T,
    {
        let len = self.slab.len();
        let index = self.slab.add_with_index(|index| {
            *unsafe { self.ids.get_unchecked_mut(len) } = MaybeUninit::new(*index);
            f(index)
        });
        index
    }

    pub fn len(&self) -> usize {
        self.slab.len()
    }

    pub fn iter(&self) -> SlabIter<T, N> {
        SlabIter { slab: self, pos: 0 }
    }

    pub fn iter_mut(&mut self) -> SlabIterMut<T, N> {
        SlabIterMut { slab: self, pos: 0 }
    }
}

pub struct SlabIter<'a, T, const N: usize> {
    slab: &'a IterableSlab<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for SlabIter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        if pos < self.slab.len() {
            let new_pos = pos + 1;
            self.pos = new_pos;
            Some(unsafe { self.slab.get_unchecked(pos) })
        } else {
            None
        }
    }
}

pub struct SlabIterMut<'a, T, const N: usize> {
    slab: &'a mut IterableSlab<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for SlabIterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        if pos < self.slab.len() {
            let new_pos = pos + 1;
            self.pos = new_pos;
            let slab: &mut Slab<T, N> = unsafe { const_transmute_unchecked(&mut *self.slab) };
            Some(unsafe { slab.get_unchecked_mut(pos) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::IterableSlab;

    #[test]
    fn simple_test() {
        let mut slab: IterableSlab<u8, 10> = IterableSlab::new();
        let id = slab
            .add_with_index(|index| {
                assert_eq!(index, &0);
                123
            })
            .unwrap();
        let value = unsafe { slab.get_unchecked_mut(id) };
        assert_eq!(&mut 123, value);
    }

    #[test]
    fn test_iter() {
        let mut slab: IterableSlab<u8, 10> = IterableSlab::new();
        slab.add_with_index(|_i| 1).unwrap();
        slab.add_with_index(|_i| 2).unwrap();
        slab.add_with_index(|_i| 3).unwrap();
        let mut iter = slab.iter();
        assert_eq!(iter.next().unwrap(), &1);
        assert_eq!(iter.next().unwrap(), &2);
        assert_eq!(iter.next().unwrap(), &3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_mut() {
        let mut slab: IterableSlab<u8, 10> = IterableSlab::new();
        slab.add_with_index(|_i| 1).unwrap();
        slab.add_with_index(|_i| 2).unwrap();
        slab.add_with_index(|_i| 3).unwrap();
        let mut iter = slab.iter_mut();
        assert_eq!(iter.next().unwrap(), &1);
        assert_eq!(iter.next().unwrap(), &2);
        assert_eq!(iter.next().unwrap(), &3);
        assert!(iter.next().is_none());
    }
}
