#![feature(generic_arg_infer)]
#![feature(const_mut_refs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod cursor;
pub mod slab;
pub mod string;
pub mod vec;

pub use cursor::*;
pub use slab::*;
pub use string::*;
pub use vec::*;

/// A const reimplementation of the [`transmute`](core::mem::transmute) function, avoiding copying
#[inline(always)]
#[doc(hidden)]
pub const unsafe fn const_transmute_unchecked<A, B>(a: A) -> B {
    use core::mem::ManuallyDrop;

    #[repr(C)]
    union Union<A, B> {
        a: ManuallyDrop<A>,
        b: ManuallyDrop<B>,
    }

    let a = ManuallyDrop::new(a);
    ManuallyDrop::into_inner(Union { a }.b)
}

pub(crate) const fn min(value: usize, value2: usize) -> usize {
    if value < value2 {
        value
    } else {
        value2
    }
}
