pub mod file_content {
    use std::env;
    use std::fs::File;
    use std::io::Read;

    #[derive(Debug)]
    pub struct FileContent {
        pub filename: String,
        pub content: Vec<String>
    }

    impl FileContent {
        pub fn open_file(&mut self) {
            let args: Vec<String> = env::args().collect();

            if args.len() < 2 {
                panic!("Not enough arguments");
            }

            let mut fp = File::open(&args[1]).unwrap();

            let mut file_contents = String::new();

            fp.read_to_string(&mut file_contents).unwrap();

            self.get_lines(&file_contents);

            self.filename = String::from(&args[1]);
        }

        fn get_lines(&self, content: &str) {
            let mut content_lines: Vec<String> = Vec::<String>::new();
            let split_char: char = '\n';
            let mut split_index: usize = 0;

            loop {
                if content.find(split_char) != None {
                    //split_index = content.find(split_char).unwrap() + 1;
                    //println!("split_index: {}", split_index);
                    split_index = content[split_index + 1..].find(split_char).unwrap();
                    println!("split_index: {}", split_index);
                    println!("{}", &content[split_index..]);
                } else { break; }
            }
        }
    }
}
