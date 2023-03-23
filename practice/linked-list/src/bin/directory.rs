use linked_list::List;
use std::env;
use std::fs;
use std::io::{self, Stdout, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct DirectoryWatcher<HT> {
    thread: Option<thread::JoinHandle<HT>>,
}

impl<HT> DirectoryWatcher<HT> {
    fn new() -> Self {
        Self { thread: None }
    }
}

fn build_list(directory: &str) -> Message {
    let mut list: List<String> = List::new();
    for entry in fs::read_dir(Path::new(directory)).unwrap().flatten() {
        list.append(entry.path().as_path().to_str().unwrap().to_string());
    }
    Message::List(list)
}

fn clear_screen(stdout: &mut Stdout) {
    stdout.write(" ".as_bytes()).unwrap();
    stdout
        .write_all(format!("{esc}c", esc = 27 as char).as_bytes())
        .unwrap();
}

#[derive(Debug)]
enum Message {
    List(List<String>),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (main_tx, main_rx) = mpsc::channel();
    let mut watcher: DirectoryWatcher<Message> = DirectoryWatcher::new();
    let mut stdout = io::stdout();

    let main_sender = main_tx;
    watcher.thread = Some(thread::spawn(move || loop {
        let list = build_list(&args[1]);
        println!("{list:?}");
        main_sender.send(list).unwrap();
        thread::sleep(Duration::from_millis(500));
    }));

    loop {
        while let Ok(data) = main_rx.recv() {
            let Message::List(list) = data;
            if !list.is_empty() {
                clear_screen(&mut stdout);
                for i in 0..list.len() {
                    let file = format!("{}\n", list.get(i).unwrap());
                    let bytes = file.as_bytes();
                    stdout.write_all(bytes).unwrap();
                }
            } else {
                clear_screen(&mut stdout);
            }
        }
    }
}
