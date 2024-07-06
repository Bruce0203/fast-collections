use generic_array::{ArrayLength, IntoArrayLength};
use typenum::Const;

use crate::{const_transmute_unchecked, Vec};

#[derive(Debug)]
pub struct String<N: ArrayLength>
where
    [u8; N::USIZE]:,
    Const<{ N::USIZE }>: IntoArrayLength,
{
    vec: Vec<u8, N>,
}

impl<N: ArrayLength> String<N>
where
    [u8; N::USIZE]:,
    Const<{ N::USIZE }>: IntoArrayLength<ArrayLength = N>,
{
    pub const fn new() -> Self {
        Self { vec: Vec::uninit() }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self {
        Self {
            vec: Vec::from_array(unsafe {
                let mut value = [32u8; N::USIZE];
                let dst: &mut [u8; L] = const_transmute_unchecked(&mut value);
                *dst = array;
                value
            }),
        }
    }

    #[inline(always)]
    pub const fn as_vec_mut(&mut self) -> &mut Vec<u8, N> {
        &mut self.vec
    }

    #[inline(always)]
    pub const fn as_vec(&self) -> &Vec<u8, N> {
        &self.vec
    }

    pub const fn len(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
mod test {
    use crate::String;

    #[test]
    fn test_array() {
        let value: String<typenum::U10> = String::from_array(*b"hell0");
        println!("{:?}", value.vec.as_slice());
        assert_eq!(value.vec.as_slice(), *b"hell0     ");
    }
}
