use rand::Rng;
use std::io;

/// Classic hot/cold number guessing game, 
/// but it's all recursive

fn main() {
    game();
}

fn guess(mut gstr: String, answer: u8) {
    let gnum: u8 = gstr.trim().parse().unwrap();
    if gnum != answer {
        match gnum {
            gnum if gnum > answer => {
                println!("Lower");
            }
            gnum if gnum < answer => {
                println!("Higher");
            }
            _ => {}
        }
        gstr.clear();
        io::stdin()
            .read_line(&mut gstr)
            .expect("Error reading input");
        guess(gstr, answer);
    } else {
        println!("Correct");
    }
}

fn play() {
    println!("Guess a number between 1 and 10");
    let a: u8 = rand::thread_rng().gen_range(1..10);
    let mut gstr = String::new();
    io::stdin()
        .read_line(&mut gstr)
        .expect("Error reading input");
    guess(gstr, a);
}

fn game() {
    println!("Would you like to play a guessing game (y/n)");
    if true {
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("Error reading input");
        c = c.trim().to_string();
        match c.as_str() {
            "y" => {
                play();
                game();
            }
            "n" => {
                std::process::exit(0);
            }
            _ => {
                println!("Valid input is (y/n)");
                game();
            }
        }
    }
}
