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

    pub const fn from_array(array: [u8; N::USIZE]) -> Self {
        Self {
            vec: Vec::from_array(array),
        }
    }
}
