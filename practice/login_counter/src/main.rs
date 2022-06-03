#![crate_name = "login_counter"]

//! Counts user logins from the login file exported by MITS.
//!
//!
//! Usage:
//! ```sh
//! login_counter [log_file] [date]
//! ```
//!

mod date;
mod logins;
use date::Date;
use logins::Logins;
use std::{collections::HashMap, env, fs, process};

fn count_logins_before_date<'a>(
    lines: Vec<Vec<&'a str>>,
    logins: &mut HashMap<&'a str, Logins>,
    date: Date,
) {
    let mut lines_before_date: Vec<Vec<&str>> = vec![];
    for line in lines {
        if !line[0].is_empty() && Date::from_str(line[0], '-') < date {
            lines_before_date.push(line.clone());
        }
    }
    count_logins(lines_before_date, logins);
}

fn count_logins<'a>(lines: Vec<Vec<&'a str>>, logins: &mut HashMap<&'a str, Logins>) {
    for line in lines {
        let name: Option<&str> = line[line.len() - 1].strip_suffix('\r');
        if let Some(n) = name {
            if logins.contains_key(n) {
                if let Some(l) = logins.get_mut(n) {
                    l.count += 1;
                    l.last_login = line[0].to_string();
                }
            } else {
                logins.insert(
                    n,
                    Logins::new(n.to_string(), line[0].to_string(), line[0].to_string(), 1),
                );
            }
        }
    }
}

fn sort_counts(counts: HashMap<&str, Logins>) -> Vec<(&str, Logins)> {
    let mut t: Vec<(&str, Logins)> = Vec::<(&str, Logins)>::new();
    for (key, val) in counts.iter() {
        t.push((key, val.clone()));
    }
    // Sort high to low
    t.sort_by(|a, b| b.1.count.partial_cmp(&a.1.count).unwrap());
    t
}

fn print_logins(counts: HashMap<&str, Logins>) {
    fn whitespace(s: &mut String, n: usize) -> &mut String {
        for _ in 0..n {
            s.push(' ');
        }
        s
    }

    let mut longest_name: usize = 0;
    let mut longest_num: usize = 0;

    for value in counts.values() {
        if value.username.len() > longest_name {
            longest_name = value.username.len();
        }
        if value.count.to_string().len() > longest_num {
            longest_num = value.count.to_string().len();
        }
    }

    let sorted_counts = sort_counts(counts);

    for (k, v) in sorted_counts.iter() {
        let mut key: String = k.to_string();
        let name = whitespace(&mut key, longest_name - k.len());
        let mut c = v.count.to_string();
        let count = whitespace(&mut c, longest_num - v.count.to_string().len());
        println!("{} {} {} {}", name, count, v.first_login, v.last_login);
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    match cli_args.len() {
        1 => {
            println!("You must provide a file name");
            process::exit(1);
        }
        2 => match cli_args[1].as_str() {
            "" => {
                println!("File name cannot be empty");
                process::exit(1);
            }
            "--help" | "-h" => {
                println!("Usage: login_counter.exe logins.log");
                process::exit(0);
            }
            _ => {
                let file_contents =
                    fs::read_to_string(&cli_args[1]).expect("Something went wrong reading file");
                let mut logins: HashMap<&str, Logins> = HashMap::<&str, Logins>::new();
                let lines: Vec<&str> = file_contents.split('\n').collect();
                let tokens: Vec<Vec<&str>> = lines.iter().map(|a| a.split(' ').collect()).collect();
                count_logins(tokens, &mut logins);
                logins.remove("bi-admin");
                print_logins(logins);
            }
        },
        3 => {
            let file_contents =
                fs::read_to_string(&cli_args[1]).expect("Something went wrong reading file");
            let mut logins: HashMap<&str, Logins> = HashMap::<&str, Logins>::new();
            let lines: Vec<&str> = file_contents.split('\n').collect();
            let tokens: Vec<Vec<&str>> = lines.iter().map(|a| a.split(' ').collect()).collect();
            count_logins_before_date(tokens, &mut logins, Date::from_str(&cli_args[2], '-'));
            logins.remove("bi-admin");
            print_logins(logins);
        }
        _ => {}
    }
}
