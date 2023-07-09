use core::ptr;


fn main() {
    let pointer = "abcdefg".as_ptr();

    unsafe {

        println!("{}", ptr::read(pointer) as char);
        println!("{}", *pointer.add(1) as char);

        println!("{}", pointer.read() as char);
        // println!("{}", *pointer.sub(1) as char);
    }
}
