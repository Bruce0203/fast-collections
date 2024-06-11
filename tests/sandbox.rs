#[test]
fn test() {
    let numbers = 0..;
    let five_numbers = numbers.take(5);
    for number in five_numbers.skip(4) {
        println!("{number}");
    }
}
