# fast_collections

0. I need a complex game system, very fast speed is required!
1. Using String, Box<T>, and Vec<T> is slow due to heap allocation.
1. In that case, let’s use &’str, which is fast even if zero copy modification of the size is impossible. However, it is difficult to manage due to its lifetime. 
2. Then there is no Cursor<T>, Vec<T>, or String<N>, but Let’s use [T;N]. However, const generics are difficult to manage 
3. If so, let’s use GenericArray. However, it lacks Vec, Cursor, and String types.
4. Then, let’s create a crate that uses typenum rather than const generic without heap allocation and no references.
