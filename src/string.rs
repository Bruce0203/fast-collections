use generic_array::{ArrayLength, IntoArrayLength};
use typenum::Const;

use crate::Vec;

#[derive(Debug)]
pub struct String<N: ArrayLength> {
    vec: Vec<u8, N>,
}

impl<N: ArrayLength> String<N> {
    pub const fn new() -> Self {
        Self { vec: Vec::uninit() }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self
    where
        Const<L>: IntoArrayLength<ArrayLength = N>,
    {
        Self {
            vec: Vec::from_array(array),
        }
    }
}
