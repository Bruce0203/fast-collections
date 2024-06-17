use std::mem::MaybeUninit;

use generic_array::{ArrayLength, GenericArray};

use crate::Vec;

pub struct String<N: ArrayLength> {
    vec: Vec<u8, N>,
}

impl<N: ArrayLength> String<N> {
    pub const fn new() -> Self {
        Self { vec: Vec::uninit() }
    }
}

impl<N> From<&'static str> for String<N>
where
    N: ArrayLength,
{
    fn from(value: &'static str) -> Self {
        String {
            vec: GenericArray::from_slice(unsafe {
                &*(value.as_bytes() as *const [u8] as *const [MaybeUninit<u8>])
            }),
        }
    }
}
