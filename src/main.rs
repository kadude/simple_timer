use notify_rust::{Notification, Timeout};
use std::env;
use std::io;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn time_left(time: u64, start: &Instant) -> u64 {
    time - start.elapsed().as_secs()
}

fn format_duration(remaining_seconds: u64) -> String {
    let hours: u64 = (remaining_seconds / (60 * 60)) % 24;
    let minutes: u64 = (remaining_seconds / 60) % 60;
    let seconds: u64 = (remaining_seconds) % 60;
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 2 {
        eprintln!("Error: Expecting one argument");
        return;
    }

    let time: u64 = args
        .iter()
        .skip(1)
        .fold(String::new(), |acc, curr| acc + " " + curr)
        .split_whitespace()
        .fold(0, |sum, value| {
            let num_value = match value {
                x if x.contains('h') => {
                    x.replace("h", "").trim().parse::<u64>().unwrap() * (60 * 60)
                }
                x if x.contains('m') => x.replace("m", "").trim().parse::<u64>().unwrap() * 60,
                x if x.contains('s') => x.replace("s", "").trim().parse::<u64>().unwrap(),
                x => x.trim().parse::<u64>().expect("Error: not an integer"),
            };
            sum + num_value
        });

    let start = Instant::now();

    while (time_left(time, &start)) != 0 {
        print!("\r");
        print!("{} ", format_duration(time_left(time, &start)));
        io::stdout().flush().expect("Could not flush stdout");
        sleep(Duration::from_millis(500));
    }
    print!("\r");
    print!("Done!");

    Notification::new()
        .summary("Time's up!")
        .body("Time to change position!")
        .icon("clock")
        .timeout(Timeout::Milliseconds(2000))
        .show()
        .unwrap();
}
