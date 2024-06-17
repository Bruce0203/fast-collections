#![feature(const_mut_refs)]

pub mod cursor;
pub mod slab;
pub mod string;
pub mod traits;
pub mod vec;

pub use cursor::*;
pub use slab::*;
pub use string::*;
pub use traits::*;
pub use vec::*;

pub mod typenum {
    pub use typenum::*;
}

pub mod generic_array {
    pub use generic_array::*;
}

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
