use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use m::csv::CSV;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    let csv = CSV::new(s);
    println!("{:#?}", csv.get_row(2));
    println!("{}", csv.sum_row(2));
    println!("{:#?}", csv.get_column(2));
    println!("{}", csv.sum_column(2));
}
