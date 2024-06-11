use generic_array::ArrayLength;

use crate::{GetUnchecked, Index, Pop, Push, RemoveUnchecked, Vec};

///Simply store element fast without any other features like get length, and iteration.
pub struct Slab<T, N: ArrayLength> {
    chunk: Vec<T, N>,
    spares: Vec<usize, N>,
}

impl<T, N: ArrayLength> Index for Slab<T, N> {
    type Index = usize;
}

impl<T, N> GetUnchecked<T> for Slab<T, N>
where
    N: ArrayLength,
{
    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        self.chunk.get_unchecked_mut(index)
    }

    unsafe fn get_unchecked(&self, index: Self::Index) -> &T {
        self.chunk.get_unchecked(index)
    }
}

impl<T, N> RemoveUnchecked for Slab<T, N>
where
    N: ArrayLength,
{
    unsafe fn remove_unchecked(&mut self, index: usize) {
        self.spares.push_unchecked(index);
    }
}

impl<T, N> Slab<T, N>
where
    N: ArrayLength,
{
    pub fn new() -> Slab<T, N> {
        Slab {
            chunk: Vec::uninit(),
            spares: Vec::uninit(),
        }
    }

    #[inline(always)]
    pub fn add_with_index<F>(&mut self, f: F) -> Result<<Self as Index>::Index, ()>
    where
        F: FnOnce(<Self as Index>::Index) -> T,
    {
        if self.spares.len() == 0 {
            let index = self.chunk.len();
            if index == N::USIZE {
                return Err(());
            }
            let elem = f(index);
            unsafe { self.chunk.push_unchecked(elem) };
            Ok(index)
        } else {
            let index = *unsafe { self.spares.pop_unchecked() };
            let elem = f(index);
            *unsafe { self.chunk.get_unchecked_mut(index) } = elem;
            Ok(index)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::GetUnchecked;

    use super::Slab;
    use generic_array::typenum::U100;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq, Eq)]
        struct A {
            inner: usize,
            index: usize,
        }
        let mut value = Slab::<A, U100>::new();
        value
            .add_with_index(|index| A { inner: 123, index })
            .unwrap();
        value
            .add_with_index(|index| A { inner: 456, index })
            .unwrap();
        assert_eq!(
            unsafe { value.get_unchecked(0) },
            &A {
                inner: 123,
                index: 0
            }
        );
        assert_eq!(
            unsafe { value.get_unchecked(1) },
            &A {
                inner: 456,
                index: 1
            }
        );
    }
}
