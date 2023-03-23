use linked_list::List;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread;
use std::time::Duration;

struct DirectoryChecker<HT> {
    thread: Option<thread::JoinHandle<HT>>,
}

impl<HT> DirectoryChecker<HT> {
    fn new() -> Self {
        Self { thread: None }
    }
}

#[derive(Debug)]
enum Message {
    List(List<String>),
}

fn main() {
    let (main_tx, main_rx) = mpsc::channel();
    let mut checker: DirectoryChecker<Message> = DirectoryChecker::new();

    let main_sender = main_tx;
    checker.thread = Some(thread::spawn(move || loop {
        let mut list: List<String> = List::new();
        for entry in fs::read_dir(Path::new("stuff")).unwrap() {
            if let Ok(file) = entry {
                list.append(file.path().as_path().to_str().unwrap().to_string());
            }
        }
        main_sender.send(Message::List(list)).unwrap();
        thread::sleep(Duration::from_millis(500));
    }));

    loop {
        while let Ok(data) = main_rx.recv() {
            let Message::List(inner) = data;
            io::stdout()
                .write(&format!("{esc}c", esc = 27 as char).as_bytes())
                .unwrap();
            if inner.len() > 0 {
                for i in 0..inner.len() {
                    io::stdout()
                        .write(&format!("{}\n", inner.get(i).unwrap()).as_bytes())
                        .unwrap();
                }
            } else {
                io::stdout()
                    .write(&format!("{esc}c", esc = 27 as char).as_bytes())
                    .unwrap();
            }
        }
    }
}
