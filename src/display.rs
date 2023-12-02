use std::{
    env, fmt,
    io::{self, Write},
    path::{Path, PathBuf},
};

use colored::{ColoredString, Colorize};

use crate::network::{DayCompletion, YearCompletion};

fn log(header: ColoredString, message: impl fmt::Display) {
    let len = header.len();
    let padding = if len >= 9 { 0 } else { 9 - len };
    eprintln!(
        "{}{}{}{}",
        " ".repeat(padding),
        header,
        ": ".dimmed(),
        message
    );
}

#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {{
        display::print_success(format!($($arg)*));
    }}
}

pub use success;

pub fn print_success(message: String) {
    log("success".green().bold(), message);
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        display::print_info(format!($($arg)*));
    }}
}

pub use info;

pub fn print_info(message: String) {
    log("info".yellow().bold(), message);
}

pub(crate) fn error(message: String) {
    log("error".red().bold(), message);
}

pub(crate) fn cause(message: String) {
    log("cause".normal(), message);
}

pub(crate) fn usage(message: String) {
    log("usage".normal(), format!("aoc {message}"));
}

pub(crate) fn or(message: String) {
    log("or".normal(), format!("aoc {message}"));
}

pub fn answer(got: &str, expected: Option<&str>, time: u64) -> bool {
    let got = Answer::new(got);
    let time = colored_time(time);
    if let Some(expected) = expected {
        let expected = Answer::new(expected);
        if got.answer == expected.answer {
            print!(
                "{}{}{}{}",
                "[".dimmed(),
                got.display().green().bold(),
                "]  ".dimmed(),
                time
            );
        } else {
            let (got, symbol) = if got.is_multiline && !expected.is_multiline {
                (got.display().yellow().bold(), "-")
            } else {
                (got.display().red().bold(), "✕")
            };
            print!(
                "{}{}{}{}{}{}{}{}",
                "[".dimmed(),
                got,
                "] ".dimmed(),
                symbol.dimmed(),
                " [".dimmed(),
                expected.display().green().bold(),
                "]  ".dimmed(),
                time
            );
        }
    } else {
        print!(
            "{}{}{}{}",
            "[".dimmed(),
            got.display().yellow().bold(),
            "]  ".dimmed(),
            time
        );
    }
    got.is_multiline
}

pub fn just_answer(answer: &str, correct: bool) {
    let answer = Answer::new(answer);
    let answer = if correct {
        answer.display().green().bold()
    } else {
        answer.display().red().bold()
    };
    println!("{}{}{}", "[".dimmed(), answer, "]".dimmed());
}

pub fn incomplete() {
    println!("{}", "incomplete".yellow());
}

pub fn wait() {
    println!("{}", "wait".yellow());
}

pub fn answer_full(
    year: &str,
    day: &str,
    part: &str,
    got: &str,
    expected: Option<&str>,
    time: u64,
) {
    day_part(year, day, part);
    if answer(got, expected, time) {
        println!();
        println!("{got}");
    } else {
        println!();
    }
}

pub fn unimplemented() {
    println!("{}", "unimplemented".yellow());
}

pub fn panic() {
    println!("{}", "panic".red());
}

pub fn panic_input(input: &str) {
    println!("{}  ({})", "panic".red(), input);
}

pub fn no_input() {
    part("*");
    println!("{}", "no input".yellow());
}

pub fn build_error() {
    part("*");
    println!("{}", "build error".red());
}

pub fn run_error() {
    println!("{}", "error".red());
}

pub fn day(year: &str, day: &str) {
    print!("{}{}{}", year, "/".dimmed(), day);
    let _ = io::stdout().flush();
}

pub fn part(part: &str) {
    print!("{}{}{}", "/".dimmed(), part, ": ".dimmed());
    let _ = io::stdout().flush();
}

pub fn day_part(year: &str, day: &str, part: &str) {
    print!(
        "{}{}{}{}{}{}",
        year,
        "/".dimmed(),
        day,
        "/".dimmed(),
        part,
        ": ".dimmed()
    );
    let _ = io::stdout().flush();
}

pub fn submit_error() {
    println!("{}", "error".red());
}

fn display_time(time: u64) -> String {
    let (div, unit) = match time {
        _ if time < 1_000_000 => (1_000, "μs"),
        _ if time < 1_000_000_000 => (1_000_000, "ms"),
        _ => (1_000_000_000, "s"),
    };
    format!("{}{unit}", time as f64 / div as f64)
}

fn colored_time(time: u64) -> ColoredString {
    let text = display_time(time);
    match time {
        _ if time < 20_000_000 => text.green(),
        _ if time < 200_000_000 => text.yellow(),
        _ => text.red(),
    }
}

pub fn stats(total_time: u64, num_parts: u8) {
    log("parts".normal(), format!("{num_parts:02}"));
    if num_parts > 0 {
        log("total".normal(), display_time(total_time));
        log(
            "average".normal(),
            display_time(total_time / num_parts as u64),
        );
    }
}

pub fn path(path: &Path) -> String {
    if path.is_absolute() {
        if let Ok(current_dir) = env::current_dir() {
            if path.starts_with(&current_dir) {
                let path_len = path.components().count();
                let current_len = current_dir.components().count();
                if path_len >= current_len.max(3) {
                    let path: PathBuf = path.components().skip(current_len - 1).collect();
                    return path.display().to_string();
                }
            }
        }
    }
    path.display().to_string()
}

struct Answer<'a> {
    answer: &'a str,
    is_multiline: bool,
}

impl<'a> Answer<'a> {
    fn new(answer: &'a str) -> Self {
        Self {
            answer,
            is_multiline: answer.lines().count() > 1,
        }
    }

    fn display(&'a self) -> &'a str {
        if self.is_multiline {
            "???"
        } else {
            self.answer
        }
    }
}

pub fn completion_header() {
    println!("{}{}{}", " ".repeat(14), "1".repeat(10), "2".repeat(6));
    println!("{}{}12345", " ".repeat(4 + 1), "1234567890".repeat(2),);
}

pub fn year_completion(year: &str, year_completion: YearCompletion) {
    print!("{year} ");
    for day in year_completion.days {
        print!(
            "{}",
            match day {
                DayCompletion::None => " ".into(),
                DayCompletion::Partial => "★".dimmed(),
                DayCompletion::Full => "★".yellow(),
            }
        );
    }
    println!(" {}", format!("{:02}", year_completion.total).yellow());
}
