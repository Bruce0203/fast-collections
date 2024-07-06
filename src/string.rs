use std::mem::MaybeUninit;

use generic_array::{ArrayLength, GenericArray, IntoArrayLength};
use typenum::Const;

use crate::{const_transmute_unchecked, Vec};

#[derive(Debug)]
pub struct String<N: ArrayLength>
where
    [u8; N::USIZE]:,
    Const<{ N::USIZE }>: IntoArrayLength,
{
    data: GenericArray<u8, N>,
}

impl<N: ArrayLength> String<N>
where
    [u8; N::USIZE]:,
    Const<{ N::USIZE }>: IntoArrayLength<ArrayLength = N>,
{
    pub const fn new() -> Self {
        Self {
            data: unsafe {
                const_transmute_unchecked(GenericArray::<MaybeUninit<u8>, N>::uninit())
            },
        }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self {
        Self {
            data: GenericArray::from_array(unsafe {
                let mut value = [32u8; N::USIZE];
                let dst: &mut [u8; L] = const_transmute_unchecked(&mut value);
                *dst = array;
                value
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::String;

    #[test]
    fn test_array() {
        let value: String<typenum::U10> = String::from_array(*b"hell0");
        println!("{:?}", value.data.as_slice());
        assert_eq!(value.data.as_slice(), *b"hell0     ");
    }
}
