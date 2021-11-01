/*
 * This example shows how Rust handles assigning variables
 * on the stack vs the heap
 */

fn main() {
    let x = 5;
    let y = x;

    // This is a "move"
    let s1 = String::from("hello");
    let s2 = s1;

    println!("x: {}, y: {}", x, y);
    // println!("{}", s1); // not allowed
    // "Borrowing after a move" this is not allowed in Rust
}
