
fn main() {
    //let cum = String::new();
    let mut cum: String = String::new();
    let a: [i32; 5] = [1,2,3,4,5];

    for num in a.iter() {
        cum += &num.to_string();
        cum += " ";
    }

    println!("{}", cum);
}
