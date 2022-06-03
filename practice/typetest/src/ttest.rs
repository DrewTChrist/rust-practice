use std::io::{StdoutLock, Write};
use std::time::{Duration, Instant};

use crossterm::{
    event::{read, Event, KeyCode::Char},
    Result,
};

pub struct TTest {
    string: String,
    errs: u16,
    elapsed: Duration,
}

impl TTest {
    pub fn new(string: String) -> Self {
        Self {
            string: string,
            errs: 0,
            elapsed: Duration::default(),
        }
    }

    pub fn default() -> Self {
        Self {
            string: String::default(),
            errs: 0,
            elapsed: Duration::default(),
        }
    }

    pub fn start(&mut self, handle: &mut StdoutLock) -> Result<()> {
        handle.write_all(self.string.as_bytes()).unwrap();
        handle.write(&[b'\n']).unwrap();
        handle.flush().unwrap();
        let mut start: Instant = Instant::now();
        let mut current_index: usize = 0;
        let mut attempts = 0;
        while current_index < self.string.len() {
            let event = read()?;
            if current_index == 0 {
                start = Instant::now();
            }
            match event {
                Event::Key(k) => match k.code {
                    Char(c) => {
                        if c != self.string.as_bytes()[current_index] as char {
                            attempts += 1;
                            if attempts == 1 {
                                self.errs += 1;
                            }
                        } else {
                            handle.write(&[c as u8]).unwrap();
                            handle.flush().unwrap();
                            current_index += 1;
                            attempts = 0;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        self.elapsed = Instant::now() - start;
        self.results(handle);
        Ok(())
    }

    pub fn reset(&mut self) {
        self.errs = 0;
        self.elapsed = Duration::default();
    }

    pub fn update_test_string(&mut self, test_string: String) {
        self.string = test_string;
    }

    fn words(&self) -> f64 {
        self.string.split(" ").collect::<Vec<&str>>().len() as f64
    }

    fn wpm(&self) -> f64 {
        self.words() / (self.elapsed.as_secs_f64() / 60.0)
    }

    fn results(&self, handle: &mut StdoutLock) {
        let mut results = String::new();
        results.push('\n');
        results.push_str(&self.errs.to_string());
        results.push_str(" errors, ");
        results.push_str(&self.acc().to_string());
        results.push_str("% accuracy, ");
        results.push_str(&self.wpm().to_string());
        results.push_str(" wpm, ");
        results.push_str(&self.elapsed.as_secs_f64().to_string());
        results.push_str(" secs\n");
        handle.write_all(results.as_bytes()).unwrap();
    }

    fn acc(&self) -> f64 {
        ((self.string.len() as f64 - self.errs as f64) / self.string.len() as f64) * 100.0
    }
}
