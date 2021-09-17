mod cli_args;
use std::fs;

fn get_contents(_args: String) -> String {
   return fs::read_to_string(&_args)
           .expect("Something went wrong reading the file");
}

fn append(_args: String, _contents: String) {
   fs::write("output".to_owned() + &_args, _contents + "hello")
       .expect("Something went wrong writing the file");
}

fn main() {
   let _file = cli_args::read_args()[1]; 
   let _contents = get_contents(_file);
   append(_file, _contents);
}
