use chrono::{Local, NaiveTime};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
    style::{Print},
    Result,
};
use std::env;
use std::io;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[cfg(target_os = "linux")]
use notify_rust::{Notification, Timeout};

fn create_ascii(number: char) -> Vec<&'static str> {
    match number {
        '1' => vec![" __ ", "/_ |", " | |", " | |", " | |", " |_|"],
        '2' => vec![" ___  ", "|__ \\ ", "   ) |", "  / / ", " / /_ ", "|____|"],
        '3' => vec![
            " ____  ", "|___ \\ ", "  __) |", " |__ < ", " ___) |", "|____/ ",
        ],
        '4' => vec![
            " _  _   ", "| || |  ", "| || |_ ", "|__   _|", "   | |  ", "   |_|  ",
        ],
        '5' => vec![
            " _____ ", "| ____|", "| |__  ", "|___ \\ ", " ___) |", "|____/ ",
        ],
        '6' => vec![
            "   __  ", "  / /  ", " / /_  ", "| '_ \\ ", "| (_) |", " \\___/ ",
        ],
        '7' => vec![
            " ______ ", "|____  |", "    / / ", "   / /  ", "  / /   ", " /_/    ",
        ],
        '8' => vec![
            "  ___  ", " / _ \\ ", "| (_) |", " > _ < ", "| (_) |", " \\___/ ",
        ],
        '9' => vec![
            "  ___  ", " / _ \\ ", "| (_) |", " \\__, |", "   / / ", "  /_/  ",
        ],
        '0' => vec![
            "  ___  ", " / _ \\ ", "| | | |", "| | | |", "| |_| |", " \\___/ ",
        ],
        ':' => vec!["   ", " _ ", "(_)", "   ", " _ ", "(_)"],
        _ => vec![""],
    }
}

fn print_ascii(ascii: Vec<&str>, start: u16) -> Result<usize> {
    let mut index = 0;
    for i in &ascii {
        execute!(stdout(), MoveTo(start, index), Print(i.to_string()))?;
        index += 1;
    }
    Ok(ascii[0].chars().count())
}

fn print_alle_ascii(verdi: String) {
    let mut start = 0;
    execute!(stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown)).ok();
    for x in verdi.chars() {
        start += print_ascii(create_ascii(x), start as u16).unwrap();
    }
}

fn time_left(time: u64, start: &Instant) -> u64 {
    time - start.elapsed().as_secs()
}

fn format_duration(remaining_seconds: u64) -> String {
    let hours: u64 = (remaining_seconds / (60 * 60)) % 24;
    let minutes: u64 = (remaining_seconds / 60) % 60;
    let seconds: u64 = (remaining_seconds) % 60;
    format!("{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
}

#[cfg(target_os = "linux")]
fn notify() {
    Notification::new()
        .summary("Time's up!")
        .body("Time to change position!")
        .icon("clock")
        .timeout(Timeout::Milliseconds(2000))
        .show()
        .unwrap();
}

#[cfg(not(target_os = "linux"))]
fn notify() {/* NOOP */}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().count() < 2 {
        eprintln!("Error: Expecting one argument");
        return;
    }

    let time: u64 = if args.iter().count() == 2 && args[1].contains(':') {
        let time_only = NaiveTime::parse_from_str(&args[1][..], "%H:%M:%S")
            .or_else(|_| NaiveTime::parse_from_str(&args[1][..], "%H:%M"))
            .ok()
            .unwrap();
        let secs = time_only
            .signed_duration_since(Local::now().time())
            .num_seconds();
        if secs < 0 {
            i64::abs((24 * 60 * 60) + secs) as u64
        } else {
            i64::abs(secs) as u64
        }
    } else {
        args.iter()
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
            })
    };

    let start = Instant::now();

    execute!(stdout(), Clear(ClearType::All)).ok();
    while (time_left(time, &start)) != 0 {
        print_alle_ascii(format_duration(time_left(time, &start)));
        io::stdout().flush().expect("Could not flush stdout");
        println!();
        sleep(Duration::from_millis(500));
    }
    print_alle_ascii(format_duration(time_left(time, &start)));
    println!();
    print!("Done!");

    notify();
}
