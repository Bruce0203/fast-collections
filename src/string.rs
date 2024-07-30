use std::fmt::{Debug, Display};

use crate::{const_transmute_unchecked, min, Vec};

#[derive(Default)]
pub struct String<const N: usize> {
    vec: Vec<u8, N>,
}

impl<const N: usize> String<N> {
    pub const fn new() -> Self {
        Self { vec: Vec::uninit() }
    }

    pub const fn from_array<const L: usize>(array: [u8; L]) -> Self {
        Self {
            vec: Vec::from_array_and_len(
                unsafe {
                    let mut value = [32u8; N];
                    let dst: &mut [u8; L] = const_transmute_unchecked(&mut value);
                    *dst = array;
                    value
                },
                const { min(L, N) },
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

    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.as_vec().as_slice()[..self.len()]) }
    }
}

impl<const N: usize> Display for String<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(std::str::from_utf8(&self.vec.as_slice()[..self.len()]).unwrap())
    }
}

impl<const N: usize> Debug for String<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(unsafe { std::str::from_utf8_unchecked(&self.vec.as_slice()[..self.len()]) })
    }
}

impl<const N: usize> Clone for String<N> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::String;

    #[test]
    fn test_array() {
        let value: String<10> = String::from_array(*b"hell0");
        println!("{:?}", value.vec.as_slice());
        assert_eq!(value.vec.as_slice(), *b"hell0     ");
        fn asdf(_value: String<5>) {}
        asdf(String::from_array(*b"a"));
        println!("{:?}", value);
    }
    #[test]
    fn asdf() {
        let value: String<4> = String::from_array(*b"123123123123");
        assert_eq!(value.len(), 4);
    }

    #[test]
    fn test_print() {
        let string = String::<100>::from_array(*b"abcd");
        assert_eq!(string.as_str(), "abcd");
    }
}
