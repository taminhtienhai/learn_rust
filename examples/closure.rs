fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let character = 'b';

    let borrow_char = || println!("Char: {}", &character);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    borrow_char();
    println!("Char: {}", character);
}