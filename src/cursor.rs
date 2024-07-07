use crate::{
    Cap, Clear, CursorRead, CursorReadTransmute, GetTransmute, GetTransmuteUnchecked, GetUnchecked,
    Index, Push, PushTransmute, PushTransmuteUnchecked, SetTransmute,
};
use core::{
    mem::{size_of, MaybeUninit},
    slice::from_raw_parts,
};
use generic_array::{ArrayLength, GenericArray};
use holder::Holdable;
use mutification::ToMut;

#[repr(C)]
#[derive(ToMut, Holdable)]
pub struct Cursor<T, N: ArrayLength> {
    buffer: GenericArray<MaybeUninit<T>, N>,
    pos: usize,
    filled_len: usize,
}

impl<T, N> Default for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer: GenericArray::uninit(),
            pos: Default::default(),
            filled_len: Default::default(),
        }
    }
}

impl<T, N> Push<T> for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn push(&mut self, item: T) -> Result<(), T> {
        if self.filled_len < self.capacity() {
            Ok(unsafe { self.push_unchecked(item) })
        } else {
            Err(item)
        }
    }

    #[inline(always)]
    unsafe fn push_unchecked(&mut self, item: T) {
        *unsafe { self.buffer.get_unchecked_mut(self.filled_len) } = MaybeUninit::new(item);
        self.filled_len = self.filled_len.unchecked_add(1);
    }
}

impl<T, N> Clear for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn clear(&mut self) {
        self.filled_len = 0;
        self.pos = 0;
    }
}

impl<T, N> Cap for Cursor<T, N>
where
    N: ArrayLength,
{
    type Cap = usize;

    #[inline(always)]
    fn capacity(&self) -> usize {
        N::USIZE
    }
}

impl<T, N> Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            buffer: GenericArray::uninit(),
            pos: 0,
            filled_len: 0,
        }
    }

    #[inline(always)]
    pub const fn pos(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    pub unsafe fn pos_mut(&mut self) -> &mut usize {
        &mut self.pos
    }

    #[inline(always)]
    pub fn filled_len(&self) -> usize {
        self.filled_len
    }

    #[inline(always)]
    pub unsafe fn filled_len_mut(&mut self) -> &mut usize {
        &mut self.filled_len
    }

    #[inline(always)]
    pub fn filled(&self) -> &[T] {
        unsafe { from_raw_parts(self.buffer.as_ptr() as *const T, self.filled_len) }
    }

    #[inline(always)]
    pub unsafe fn filled_mut(&mut self) -> &mut [T] {
        core::slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut T, self.filled_len)
    }

    #[inline(always)]
    pub fn unfilled(&self) -> &[T] {
        unsafe {
            from_raw_parts(
                (self.buffer.as_ptr() as *const T).offset(self.filled_len as isize),
                N::USIZE - self.filled_len,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn unfilled_mut(&mut self) -> &mut [T] {
        core::slice::from_raw_parts_mut(
            (self.buffer.as_mut_ptr() as *mut T).offset(self.filled_len as isize),
            N::USIZE - self.filled_len,
        )
    }
}

impl<T, N> CursorRead<T> for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn read(&mut self) -> Option<&T> {
        let pos = self.pos().clone();
        if pos < self.filled_len {
            unsafe { *self.pos_mut() = pos.unchecked_add(1) };
            self.buffer.get(pos).map(|v| unsafe { v.assume_init_ref() })
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn read_unchecked(&mut self) -> &T {
        let pos = self.pos().clone();
        *self.pos_mut() = pos.unchecked_add(1);
        self.buffer.get_unchecked(pos).assume_init_ref()
    }
}

impl<T, N> GetTransmute for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn get_transmute<V>(&self, index: Self::Index) -> Option<&V> {
        if index < N::USIZE as Self::Index {
            Some(unsafe { self.get_transmute_unchecked(index) })
        } else {
            None
        }
    }

    #[inline(always)]
    fn get_transmute_mut<V>(&mut self, index: Self::Index) -> Option<&mut V> {
        if index < N::USIZE as Self::Index {
            Some(unsafe { self.get_transmute_mut_unchecked(index) })
        } else {
            None
        }
    }
}

impl<T, N> GetTransmuteUnchecked for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    unsafe fn get_transmute_unchecked<V>(&self, index: Self::Index) -> &V {
        let value = self as *const Self as *const u8;
        &*value.offset(index as isize).cast::<V>()
    }

    #[inline(always)]
    unsafe fn get_transmute_mut_unchecked<V>(&mut self, index: Self::Index) -> &mut V {
        let value = self as *mut Self as *mut u8;
        &mut *value.offset(index as isize).cast::<V>()
    }
}

const fn assert_types<T, V>() {
    assert!(core::mem::size_of::<V>() >= core::mem::size_of::<T>())
}

const fn calc_index_from_input_size_and_unit_isze<T, V>() -> usize {
    let input_size = core::mem::size_of::<V>();
    let unit_size = core::mem::size_of::<T>();
    if input_size % unit_size != 0 {
        input_size / unit_size + 1
    } else {
        input_size / unit_size
    }
}

impl<T, N> PushTransmute for Cursor<T, N>
where
    N: ArrayLength,
{
    fn push_transmute<V>(&mut self, value: V) -> Result<(), ()> {
        if size_of::<V>() + self.pos < N::USIZE {
            Ok(unsafe { self.push_transmute_unchecked(value) })
        } else {
            Err(())
        }
    }
}

impl<T, V, N> PushTransmuteUnchecked<V> for Cursor<T, N>
where
    T: Sized,
    N: ArrayLength,
{
    unsafe fn push_transmute_unchecked(&mut self, value: V) {
        const { assert_types::<T, V>() };
        let ptr = self as *mut Self as *mut u8;
        *ptr.offset(self.filled_len as isize).cast::<V>() = value;
        *self.filled_len_mut() = self
            .filled_len
            .unchecked_add(const { calc_index_from_input_size_and_unit_isze::<T, V>() });
    }
}

impl<N> CursorReadTransmute for Cursor<u8, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    fn read_transmute<T>(&mut self) -> Option<&T> {
        if self.pos() + core::mem::size_of::<T>() > self.filled_len() {
            None
        } else {
            Some(unsafe { self.read_transmute_unchecked() })
        }
    }

    #[inline(always)]
    unsafe fn read_transmute_unchecked<T>(&mut self) -> &T {
        let value = self as *const Self as *const u8;
        let result = &*value.offset(self.pos as isize).cast::<T>();
        *self.pos_mut() = self.pos.unchecked_add(core::mem::size_of::<T>());
        result
    }
}

impl<T, N> Index for Cursor<T, N>
where
    N: ArrayLength,
{
    type Index = usize;
}

impl<T, N> GetUnchecked<T> for Cursor<T, N>
where
    N: ArrayLength,
{
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: Self::Index) -> &T {
        self.buffer.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: Self::Index) -> &mut T {
        self.buffer.get_unchecked_mut(index).assume_init_mut()
    }
}

impl<T, N> SetTransmute for Cursor<T, N>
where
    N: ArrayLength,
{
    fn set_transmute<V>(&mut self, index: usize, value: V) -> Result<(), ()> {
        if index + core::mem::size_of::<V>() < N::USIZE {
            unsafe { Ok(self.set_transmute_unchecked(index, value)) }
        } else {
            Err(())
        }
    }

    unsafe fn set_transmute_unchecked<V>(&mut self, index: usize, value: V) {
        const { assert_types::<T, V>() };
        let ptr = self as *mut Self as *mut u8;
        *ptr.offset(index as isize).cast::<V>() = value;
    }
}

#[cfg(test)]
mod test {
    use crate::{CursorRead, CursorReadTransmute, Push};

    use super::Cursor;
    use generic_array::typenum::{U100, U8};

    #[test]
    fn test() {
        let mut buffer: Cursor<u8, U100> = Cursor::new();
        for i in 1..5 {
            buffer.push(i).unwrap();
        }
        assert_eq!(buffer.filled_len(), 4);
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct A {
            v0: u8,
            v1: u8,
        }
        let value: &A = buffer.read_transmute().unwrap();
        assert_eq!(value, &A { v0: 1, v1: 2 });
        let value: &A = buffer.read_transmute().unwrap();
        assert_eq!(value, &A { v0: 3, v1: 4 });
    }

    #[test]
    fn filled() {
        let mut buffer: Cursor<u8, U8> = Cursor::new();
        for i in 1..9 {
            buffer.push(i).unwrap();
        }
        unsafe { *buffer.filled_len_mut() = 4 };
        assert_eq!(unsafe { buffer.filled() }, &[1, 2, 3, 4]);
        assert_eq!(unsafe { buffer.filled_mut() }, &[1, 2, 3, 4]);
        assert_eq!(unsafe { buffer.unfilled() }, &[5, 6, 7, 8]);
        assert_eq!(unsafe { buffer.unfilled_mut() }, &[5, 6, 7, 8]);
        unsafe { *buffer.pos_mut() = 2 }
        assert_eq!(buffer.read().unwrap(), &3);
        unsafe { *buffer.pos_mut() = 1 }
        assert_eq!(buffer.read_transmute::<[u8; 2]>().unwrap(), &[2, 3]);
        assert_eq!(buffer.read_transmute::<[u8; 2]>(), None);
    }
}
