fn main() {
    let s = String::from("hello");

    let slice = &s[0..2];

    // let slice = &s[..2]; // same as above

    println!("{}", slice);
}
