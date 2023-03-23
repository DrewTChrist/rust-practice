//use linked_list::List;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread;
use std::time::Duration;

macro_rules! new {
    () => {
        fn new() -> Self {
            let (tx, rx) = mpsc::channel();
            Self {
                sender: tx,
                receiver: rx,
                thread: None,
            }
        }
    };
}

struct Collector<T, HT> {
    sender: Sender<T>,
    receiver: Receiver<T>,
    thread: Option<thread::JoinHandle<HT>>,
}

impl<T, HT> Collector<T, HT> {
    new!();
}

struct DirectoryChecker<T, HT> {
    sender: Sender<T>,
    receiver: Receiver<T>,
    thread: Option<thread::JoinHandle<HT>>,
}

impl<T, HT> DirectoryChecker<T, HT> {
    new!();
}

#[derive(Debug)]
enum Message {
    //List(List<String>),
    List(Vec<String>),
    Str(String),
    Empty,
}

fn main() {
    println!("Starting Main");
    let (main_tx, main_rx) = mpsc::channel();
    let mut collector = Collector::new();
    let mut checker: DirectoryChecker<Message, _> = DirectoryChecker::new();

    let collector_sender = collector.sender;
    checker.thread = Some(thread::spawn(move || {
        println!("Checker thread");
        loop {
            //let mut list: List<String> = List::new();
            let mut list: Vec<String> = Vec::new();
            for entry in fs::read_dir(Path::new("stuff")).unwrap() {
                if let Ok(file) = entry {
                    //list.append(file.path().as_path().to_str().unwrap().to_string());
                    list.push(file.path().as_path().to_str().unwrap().to_string());
                }
            }
            collector_sender.send(Message::List(list)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    let main_sender = main_tx;
    collector.thread = Some(thread::spawn(move || {
        println!("Collector thread");
        loop {
            let mut history: Vec<String> = Vec::new();
            if let Ok(data) = collector.receiver.recv() {
                if let Message::List(list) = data {
                    for item in list {
                        //println!("{:?}", item);
                        if !history.contains(&item) {
                            history.push(item);
                        }
                    }
                }
            }
            main_sender.send(Message::List(history)).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }));

    loop {
        while let Ok(data) = main_rx.recv() {
            if let Message::List(inner) = data {
                io::stdout()
                    .write(&format!("{esc}c", esc = 27 as char).as_bytes())
                    .unwrap();
                if inner.len() > 0 {
                    for item in inner {
                        io::stdout()
                            .write(&format!("{}\n", item).as_bytes())
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
}
