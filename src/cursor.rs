use core::{mem::MaybeUninit, slice::from_raw_parts};
use generic_array::{ArrayLength, GenericArray};

#[repr(C)]
pub struct Cursor<T, N: ArrayLength> {
    buffer: GenericArray<MaybeUninit<T>, N>,
    pos: usize,
    filled: usize,
}

impl<T, N: ArrayLength> Cursor<T, N> {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            buffer: GenericArray::uninit(),
            pos: 0,
            filled: 0,
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
    pub const fn filled_len(&self) -> usize {
        self.filled
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        N::USIZE
    }

    #[inline(always)]
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.filled < self.capacity() {
            Ok(unsafe { self.push_unchecked(item) })
        } else {
            Err(item)
        }
    }

    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, item: T) {
        *unsafe {
            self.buffer.get_unchecked_mut({
                let filled = self.filled.clone();
                self.filled = self.filled.unchecked_add(1);
                filled
            })
        } = MaybeUninit::new(item);
    }

    #[inline(always)]
    pub unsafe fn set_filled_len(&mut self, filled: usize) {
        self.filled = filled;
    }

    #[inline(always)]
    pub unsafe fn filled(&self) -> &[T] {
        from_raw_parts(self.buffer.as_ptr() as *const T, self.filled)
    }

    #[inline(always)]
    pub unsafe fn filled_mut(&mut self) -> &mut [T] {
        core::slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut T, self.filled)
    }

    #[inline(always)]
    pub unsafe fn unfilled(&self) -> &[T] {
        from_raw_parts(
            (self.buffer.as_ptr() as *const T).offset(self.filled as isize),
            N::USIZE - self.filled,
        )
    }

    #[inline(always)]
    pub unsafe fn unfilled_mut(&mut self) -> &mut [T] {
        core::slice::from_raw_parts_mut(
            (self.buffer.as_mut_ptr() as *mut T).offset(self.filled as isize),
            N::USIZE - self.filled,
        )
    }
}

#[cfg(test)]
mod test {
    use super::Cursor;
    use crate::read::GenericRead;
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
        let value: &A = buffer.read_type().unwrap();
        assert_eq!(value, &A { v0: 1, v1: 2 });
        let value: &A = buffer.read_type().unwrap();
        assert_eq!(value, &A { v0: 3, v1: 4 });
    }

    #[test]
    fn filled() {
        let mut buffer: Cursor<u8, U8> = Cursor::new();
        for i in 1..9 {
            buffer.push(i).unwrap();
        }
        unsafe { buffer.set_filled_len(4) };
        assert_eq!(unsafe { buffer.unfilled() }, &[5, 6, 7, 8]);
    }
}
