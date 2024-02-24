#![warn(rust_2018_idioms)]
#![warn(clippy::pedantic)]

use std::env;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use chrono::{DateTime, Local};

fn main() {
    if let Some(fmt) = env::args().nth(1) {
        let duration = Duration::from_secs(1);
        loop {
            let now: DateTime<Local> = Local::now();
            println!("{}", now.format(&fmt));

            sleep(duration);
        }
    } else {
        eprintln!("usage: {} <format>", env::args().next().unwrap());
        exit(1);
    }
}
