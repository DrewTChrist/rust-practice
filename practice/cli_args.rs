use std::env;

pub fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    return args;
}
