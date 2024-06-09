use generic_array::ArrayLength;

use crate::Vec;

pub struct String<N: ArrayLength> {
    data: Vec<u8, N>,
}

impl<N: ArrayLength> String<N> {
    pub const fn new() -> Self {
        Self {
            data: Vec::uninit(),
        }
    }
}
