use crate::{
    const_transmute_unchecked, Cap, Clear, CursorRead, CursorReadTransmute, GetTransmute,
    GetTransmuteUnchecked, GetUnchecked, Index, Push, PushTransmute, PushTransmuteUnchecked,
    SetTransmute,
};
use core::{
    mem::{size_of, MaybeUninit},
    slice::from_raw_parts,
};

#[repr(C)]
pub struct Cursor<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    pos: usize,
    filled_len: usize,
}

impl<T, const N: usize> Default for Cursor<T, N> {
    #[inline(always)]
    fn default() -> Self {
        Self {
            buffer: [const { MaybeUninit::uninit() }; N],
            pos: Default::default(),
            filled_len: Default::default(),
        }
    }
}

impl<T, const N: usize> Push<T> for Cursor<T, N> {
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

impl<T, const N: usize> Clear for Cursor<T, N> {
    #[inline(always)]
    fn clear(&mut self) {
        self.filled_len = 0;
        self.pos = 0;
    }
}

impl<T, const N: usize> Cap for Cursor<T, N> {
    type Cap = usize;

    #[inline(always)]
    fn capacity(&self) -> usize {
        N
    }
}

impl<const N: usize> Cursor<u8, N> {
    ///Clear src fill dst
    pub fn push_from_cursor<const N2: usize>(
        &mut self,
        src: &mut Cursor<u8, N2>,
    ) -> Result<(), ()> {
        let dst = self;
        let src_filled_len = src.filled_len();
        let dst_filled_len = *unsafe { dst.filled_len_mut() };
        if src_filled_len + dst_filled_len < N {
            let unfilled = unsafe { dst.unfilled_mut() };
            let src_pos = src.pos();
            unfilled[..src_filled_len].copy_from_slice(&src.as_array()[src_pos..src_filled_len]);
            unsafe { *dst.filled_len_mut() = dst_filled_len.unchecked_add(src_filled_len) };
            src.clear();
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<T, const N: usize> Cursor<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            buffer: [const { MaybeUninit::uninit() }; N],
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
                N - self.filled_len,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn unfilled_mut(&mut self) -> &mut [T] {
        core::slice::from_raw_parts_mut(
            (self.buffer.as_mut_ptr() as *mut T).offset(self.filled_len as isize),
            N - self.filled_len,
        )
    }

    #[inline(always)]
    pub const fn as_array(&mut self) -> &mut [T; N] {
        unsafe { const_transmute_unchecked(self) }
    }

    #[inline(always)]
    pub const fn remaining(&self) -> usize {
        self.filled_len - self.pos
    }
}

impl<T, const N: usize> CursorRead<T> for Cursor<T, N> {
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

impl<'a, T, const N: usize> GetTransmute<'a> for Cursor<T, N> {
    #[inline(always)]
    fn get_transmute<V>(&self, index: Self::Index) -> Option<&V> {
        if index < N as Self::Index {
            Some(unsafe { self.get_transmute_unchecked(index) })
        } else {
            None
        }
    }

    #[inline(always)]
    fn get_transmute_mut<V>(&mut self, index: Self::Index) -> Option<&mut V> {
        if index < N as Self::Index {
            Some(unsafe { self.get_transmute_mut_unchecked(index) })
        } else {
            None
        }
    }
}

impl<'a, T, const N: usize> GetTransmuteUnchecked<'a> for Cursor<T, N> {
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

impl<T, const N: usize> PushTransmute for Cursor<T, N> {
    fn push_transmute<V>(&mut self, value: V) -> Result<(), ()> {
        if size_of::<V>() + self.filled_len < N {
            Ok(unsafe { self.push_transmute_unchecked(value) })
        } else {
            Err(())
        }
    }
}

impl<T, V, const N: usize> PushTransmuteUnchecked<V> for Cursor<T, N>
where
    T: Sized,
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

impl<const N: usize> CursorReadTransmute for Cursor<u8, N> {
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

impl<'a, T, const N: usize> Index<'a> for Cursor<T, N> {
    type Index = usize;
}

impl<'a, T, const N: usize> GetUnchecked<'a, T> for Cursor<T, N> {
    #[inline(always)]
    unsafe fn get_unchecked(&self, index: Self::Index) -> &T {
        self.buffer.get_unchecked(index).assume_init_ref()
    }

    #[inline(always)]
    unsafe fn get_unchecked_mut(&mut self, index: Self::Index) -> &mut T {
        self.buffer.get_unchecked_mut(index).assume_init_mut()
    }
}

impl<T, const N: usize> SetTransmute for Cursor<T, N> {
    fn set_transmute<V>(&mut self, index: usize, value: V) -> Result<(), ()> {
        if index + core::mem::size_of::<V>() < N {
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

impl<T: Copy, const N: usize> Clone for Cursor<T, N> {
    fn clone(&self) -> Self {
        let mut cursor = Self {
            buffer: [MaybeUninit::uninit(); N],
            pos: self.pos.clone(),
            filled_len: self.filled_len.clone(),
        };
        cursor.buffer.copy_from_slice(&self.buffer.as_slice());
        cursor
    }
}

#[cfg(feature = "std")]
impl<const N: usize> Cursor<u8, N> {
    pub fn push_from_read<R: std::io::Read>(&mut self, read: &mut R) -> std::io::Result<()> {
        let unfilled = unsafe { self.unfilled_mut() };
        let read_length = read.read(unfilled)?;
        unsafe { *self.filled_len_mut() += read_length };
        Ok(())
    }

    pub fn push_to_write<W: std::io::Write>(&mut self, write: &mut W) -> std::io::Result<()> {
        write.write_all(self.filled())?;
        self.clear();
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::{const_transmute_unchecked, CursorRead, CursorReadTransmute, Push, PushTransmute};

    use super::Cursor;
    use rand::Rng;

    #[test]
    fn test() {
        let mut buffer: Cursor<u8, 100> = Cursor::new();
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
        let mut buffer: Cursor<u8, 8> = Cursor::new();
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

    #[test]
    fn test_cursor_push() {
        let mut cursor: Cursor<u8, 100> = Cursor::new();
        let value: [u8; 2] = unsafe { const_transmute_unchecked(100u16) };
        cursor.push_transmute(260u16).unwrap();
        assert_eq!(cursor.filled_len(), 2);
        assert_eq!(cursor.read_transmute::<u16>().unwrap(), &260u16);
    }

    #[test]
    fn test_cursor_copy_from_cursor() {
        let mut dst: Cursor<u8, 100> = Cursor::new();
        let mut src: Cursor<u8, 100> = Cursor::new();
        let value: usize = rand::thread_rng().gen();
        src.push_transmute(value).unwrap();
        dst.push_from_cursor(&mut src).unwrap();
        assert_eq!(dst.read_transmute::<usize>().unwrap(), &value);
    }
}
