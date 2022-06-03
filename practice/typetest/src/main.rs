mod ttest;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::io::stdout;
use std::ops::Range;
use ttest::TTest;
use rand::Rng;

fn rand_test(wc: u32, wlmin: u32, wlmax: u32) -> String {
    let mut s: String = String::default();
    for i in 1..wc {
        let wl = rand::thread_rng().gen_range(wlmin..wlmax);
        for _j in 1..wl {
            let ch = rand::thread_rng().gen_range::<u8, Range<u8>>(97..122) as char;
            s.push(ch);
        }
        if i < wc - 1 {
            s.push(' ');
        }
    }
    return s;
}

fn main() -> Result<()> {
    enable_raw_mode().unwrap();

    let stdout = stdout();

    let mut handle = stdout.lock();

    let tests: [String; 1] = [
        //String::from("one 2 three 4 five 6 seven 8 nine 10"),
        //String::from("hello there"),
        rand_test(10, 5, 10),
    ];

    let mut ttest = TTest::default();

    for t in tests {
        ttest.reset();
        ttest.update_test_string(t);

        ttest.start(&mut handle).unwrap();

        disable_raw_mode().unwrap();
    }

    Ok(())
}
