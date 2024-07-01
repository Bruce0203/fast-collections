use std::mem::MaybeUninit;

use generic_array::{ArrayLength, GenericArray, IntoArrayLength};
use typenum::Const;

use crate::{const_transmute_unchecked, Vec};

#[derive(Debug)]
pub struct String<N: ArrayLength> {
    data: GenericArray<u8, N>,
}

impl<N: ArrayLength> String<N> {
    pub const fn new() -> Self {
        Self {
            data: unsafe {
                const_transmute_unchecked(GenericArray::<MaybeUninit<u8>, N>::uninit())
            },
        }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self
    where
        Const<L>: IntoArrayLength<ArrayLength = N>,
    {
        Self {
            data: GenericArray::from_array(array),
        }
    }
}
