fn main() {
    another_function(5, 25);
    println!("five: {}", five());
    println!("5 + 1: {}", plus_one(5));
    println!("With return: {}", with_return());
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}:", x);
    println!("The value of y is {}:", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // <- This actually returns x + 1 (expression)
    //x + 1; <- This would give an error (statement)
}

fn with_return() -> i32 {
    return 42;
}
