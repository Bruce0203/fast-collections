use typenum::{Diff, Integer, PInt, Sum, U100, U30};

#[test]
fn example() {
    type X = Diff<PInt<U100>, PInt<U30>>;
    let value = <X as Integer>::ISIZE;
    println!("{:?}", value);
}
