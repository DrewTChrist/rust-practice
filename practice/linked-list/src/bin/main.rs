use std::process::Command;
use std::sync::{mpsc, mpsc::Receiver, mpsc::Sender};
use std::thread;

use linked_list::List;

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

struct Commander<T, HT> {
    sender: Sender<T>,
    receiver: Receiver<T>,
    thread: Option<thread::JoinHandle<HT>>,
}

impl<T, HT> Commander<T, HT> {
    new!();
}

#[derive(Debug)]
enum Message {
    List(List<String>),
    Str(String),
    Empty,
}

fn main() {
    let (main_tx, main_rx) = mpsc::channel();
    let mut collector = Collector::new();
    let mut commander: Commander<Message, _> = Commander::new();

    let collector_sender = collector.sender;
    commander.thread = Some(thread::spawn(move || {
        for i in 1..11 {
            let val = Command::new("sh")
                .args(["-c", format!("cat stuff/{}.txt", i).as_str()])
                .output()
                .unwrap();
            if val.stdout.len() > 0 {
                collector_sender
                    .send(Message::Str(String::from_utf8(val.stdout).unwrap()))
                    .unwrap();
            } else {
                collector_sender.send(Message::Empty).unwrap();
            }
        }
    }));

    let main_sender = main_tx;
    collector.thread = Some(thread::spawn(move || {
        let mut l: List<String> = List::new();
        while let Ok(data) = collector.receiver.recv() {
            if let Message::Str(inner) = data {
                l.append(inner);
            }
        }
        main_sender.send(Message::List(l)).unwrap();
    }));

    let mut list: Option<List<String>> = None;

    while let Ok(data) = main_rx.recv() {
        if let Message::List(inner) = data {
            list = Some(inner);
            //for i in 0..inner.len() {
            //    println!("{:?}", inner.get(i).unwrap());
            //}
        } else {
            list = None;
        }
    }

    println!("{:?}", list.unwrap());
}
