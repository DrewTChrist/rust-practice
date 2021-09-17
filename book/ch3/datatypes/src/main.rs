fn main() {

    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32

    let _sum = 5 + 10; // addition

    let _diff = 95.5 - 4.3; // subtraction

    let _product = 4 * 30; // multiplication

    let _quotient = 56.7 / 32.2; // division

    let _remainder = 43 % 5; // modulus

    let _t = true; // implicit boolean

    let _f: bool = false; // boolean with explicit type annotation

    let _c = 'z'; // character

    let _z = 'Z'; //character

    let heart_eyed_cat = '😻'; // character

    let tup: (i32, f64, u8) = (500, 6.4, 1); // declare a tuple

    let (x, y, z) = tup; // pattern matching

    // println!("The value of y is: {}", y); // result

    let new_tup: (i32, f64, u8) = (500, 6.4, 1); // tuple

    let five_hundred = new_tup.0; // access tuple by index with a dot

    let six_point_four = new_tup.1;

    let one = new_tup.2;

    let arr = [1, 2, 3, 4, 5]; // array
}
