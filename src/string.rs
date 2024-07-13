use std::fmt::{Debug, Display, Error};

use generic_array::{ArrayLength, IntoArrayLength};
use typenum::Const;

use crate::{const_transmute_unchecked, Vec};

#[derive(Default)]
pub struct String<N: ArrayLength> {
    vec: Vec<u8, N>,
}

impl<N: ArrayLength> String<N> {
    pub const fn new() -> Self {
        Self { vec: Vec::uninit() }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self
    where
        Const<{ N::USIZE }>: IntoArrayLength<ArrayLength = N>,
    {
        Self {
            vec: Vec::from_array_and_len(
                unsafe {
                    let mut value = [32u8; N::USIZE];
                    let dst: &mut [u8; L] = const_transmute_unchecked(&mut value);
                    *dst = array;
                    value
                },
                L,
            ),
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

impl<N> Display for String<N>
where
    N: ArrayLength,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(self.vec.as_slice()).unwrap())
    }
}

impl<N> Debug for String<N>
where
    N: ArrayLength,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(self.vec.as_slice()) })
    }
}

impl<N> Clone for String<N>
where
    N: ArrayLength,
{
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use typenum::U5;

    use crate::String;

    #[test]
    fn test_array() {
        let value: String<typenum::U10> = String::from_array(*b"hell0");
        println!("{:?}", value.vec.as_slice());
        assert_eq!(value.vec.as_slice(), *b"hell0     ");
        fn asdf(value: String<U5>) {}
        asdf(String::from_array(*b"a"));
        println!("{:?}", value);
    }
}
