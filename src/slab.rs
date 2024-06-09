use generic_array::ArrayLength;

use crate::Vec;

///Simply store element fast without any other features like get length, and iteration.
pub struct Slab<T, N: ArrayLength> {
    values_fragmented_by_index: Vec<T, N>,
    spares: Vec<usize, N>,
}

impl<T, N: ArrayLength> Slab<T, N> {
    pub fn new() -> Slab<T, N> {
        Slab {
            values_fragmented_by_index: Vec::uninit(),
            spares: Vec::uninit(),
        }
    }

    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    pub unsafe fn get(&mut self, index: usize) -> &mut T {
        self.values_fragmented_by_index.get_unchecked_mut(index)
    }

    #[inline(always)]
    pub fn add<F>(&mut self, f: F) -> Result<usize, ()>
    where
        F: FnOnce(usize) -> T,
    {
        if self.spares.len() == 0 {
            let index = self.values_fragmented_by_index.len();
            if index == N::USIZE {
                return Err(());
            }
            let elem = f(index);
            unsafe { self.values_fragmented_by_index.push_unchecked(elem) };
            Ok(index)
        } else {
            let index = *unsafe { self.spares.pop_unchecked() };
            let elem = f(index);
            *unsafe { self.values_fragmented_by_index.get_unchecked_mut(index) } = elem;
            Ok(index)
        }
    }

    ///After removing an element, be cautious as you might still unintentionally access it using [Self::get_unchecked_mut].
    pub unsafe fn remove(&mut self, index: usize) {
        self.spares.push_unchecked(index);
    }
}

#[cfg(test)]
mod test {
    use super::Slab;
    use generic_array::typenum::U100;

    #[test]
    fn test() {
        struct A {
            inner: usize,
            index: usize,
        }
        let mut value = Slab::<A, U100>::new();
        value.add(|index| A { inner: 123, index }).unwrap();
        value.add(|index| A { inner: 456, index }).unwrap();
        assert_eq!(unsafe { (value.get(0).inner, 0) }, (123, 0));
        assert_eq!(unsafe { (value.get(1).inner, 1) }, (456, 1));
    }
}
