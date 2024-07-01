pub trait Index {
    type Index;
}

pub trait Pop<T> {
    fn pop(&mut self) -> Option<&T>;
    unsafe fn pop_unchecked(&mut self) -> &T;

    fn pop_mut(&mut self) -> Option<&mut T>;
    unsafe fn pop_unchecked_mut(&mut self) -> &mut T;
}

pub trait Get<T>: Index {
    fn get(&self, index: Self::Index) -> Option<&T>;
    fn get_mut(&mut self, index: Self::Index) -> Option<&mut T>;
}

pub trait GetUnchecked<T>: Index {
    unsafe fn get_unchecked_ref(&self, index: Self::Index) -> &T;
    unsafe fn get_unchecked_mut(&mut self, index: Self::Index) -> &mut T;
}

pub trait Push<T> {
    fn push(&mut self, value: T) -> Result<(), T>;
    unsafe fn push_unchecked(&mut self, value: T);
}

pub trait Remove: Index {
    fn remove(&mut self, index: Self::Index) -> bool;
}

pub trait RemoveUnchecked: Index {
    unsafe fn remove_unchecked(&mut self, index: Self::Index);
}

pub trait Cap {
    type Cap;

    fn capacity(&self) -> Self::Cap;
}

pub trait CursorRead<T> {
    fn read(&mut self) -> Option<&T>;
    unsafe fn read_unchecked(&mut self) -> &T;
}

pub trait CursorReadTransmute {
    fn read_transmute<T>(&mut self) -> Option<&T>;
    unsafe fn read_transmute_unchecked<T>(&mut self) -> &T;
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait GetTransmute: Index {
    fn get_transmute<V>(&self, index: Self::Index) -> Option<&V>;
    fn get_transmute_mut<V>(&mut self, index: Self::Index) -> Option<&mut V>;
}

pub trait GetTransmuteUnchecked: Index {
    unsafe fn get_transmute_unchecked<V>(&self, index: Self::Index) -> &V;
    unsafe fn get_transmute_mut_unchecked<V>(&mut self, index: Self::Index) -> &mut V;
}
