use core::mem::size_of;

use generic_array::ArrayLength;

use crate::Cursor;

pub trait GenericRead {
    fn read_type<T>(&mut self) -> Option<&T>;
    unsafe fn read_type_unchecked<T>(&mut self) -> &T;
}

impl<N: ArrayLength> GenericRead for Cursor<u8, N> {
    fn read_type<T>(&mut self) -> Option<&T> {
        if self.pos() + size_of::<T>() > self.filled_len() {
            None
        } else {
            Some(unsafe { self.read_type_unchecked() })
        }
    }

    #[inline(always)]
    unsafe fn read_type_unchecked<T>(&mut self) -> &T {
        &*(self as *const Self as *const T).byte_offset({
            let pos = self.pos().clone();
            *self.pos_mut() += size_of::<T>();
            pos as isize
        } as isize)
    }
}
