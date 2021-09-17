
fn main() {
    //let cum = String::new();
    let mut cumulative: String = String::new();
    let a: [i32; 5] = [1,2,3,4,5];

    for num in a.iter() {
        cumulative += &num.to_string();
        cumulative += " ";
    }

    println!("{}", cumulative);
}
