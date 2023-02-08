use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use std::{
    env, fmt,
    io::{self, Write},
    path::{Path, PathBuf},
};

macro_rules! color {
    ($($arg:expr),*) => {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        $(
            let colored: Colored<_> = $arg.into();
            let _ = stdout.set_color(&colored.color);
            let _ = write!(&mut stdout, "{}", colored.value);
        )*
        let _ = stdout.reset();
    };
}

macro_rules! colorln {
    ($($arg:expr),*) => {
        color!($($arg),*);
        println!();
    };
}

macro_rules! ecolor {
    ($($arg:expr),*) => {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        $(
            let colored: Colored<_> = $arg.into();
            let _ = stderr.set_color(&colored.color);
            let _ = write!(&mut stderr, "{}", colored.value);
        )*
        let _ = stderr.reset();
    };
}

macro_rules! ecolorln {
    ($($arg:expr),*) => {
        ecolor!($($arg),*);
        eprintln!();
    };
}

fn log(header: Colored<&str>, message: impl fmt::Display) {
    let len = header.value.len();
    let padding = if len >= 9 { 0 } else { 9 - len };
    ecolorln!(" ".repeat(padding), header, ": ".dim(), message);
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
            color!("[".dim(), got.display().green().bold(), "]  ".dim(), time);
        } else {
            let (got, symbol) = if got.is_multiline && !expected.is_multiline {
                (got.display().yellow().bold(), "-")
            } else {
                (got.display().red().bold(), "✕")
            };
            color!(
                "[".dim(),
                got,
                "] ".dim(),
                symbol.dim(),
                " [".dim(),
                expected.display().green().bold(),
                "]  ".dim(),
                time
            );
        }
    } else {
        color!("[".dim(), got.display().yellow().bold(), "]  ".dim(), time);
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
    colorln!("[".dim(), answer, "]".dim());
}

pub fn wait() {
    colorln!("wait".yellow());
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
    colorln!("unimplemented".yellow());
}

pub fn panic() {
    colorln!("panic".red());
}

pub fn panic_input(input: &str) {
    colorln!("panic".red(), "  (", input, ")");
}

pub fn no_input() {
    part("*");
    colorln!("no input".yellow());
}

pub fn build_error() {
    part("*");
    colorln!("build error".red());
}

pub fn day(year: &str, day: &str) {
    color!(year, "/".dim(), day);
    let _ = io::stdout().flush();
}

pub fn part(part: &str) {
    color!("/".dim(), part, ": ".dim());
    let _ = io::stdout().flush();
}

pub fn day_part(year: &str, day: &str, part: &str) {
    color!(year, "/".dim(), day, "/".dim(), part, ": ".dim());
    let _ = io::stdout().flush();
}

pub fn submit_error() {
    colorln!("error".red());
}

fn display_time(time: u64) -> String {
    let (div, unit) = match time {
        _ if time < 1_000_000 => (1_000, "μs"),
        _ if time < 1_000_000_000 => (1_000_000, "ms"),
        _ => (1_000_000_000, "s"),
    };
    format!("{}{unit}", time as f64 / div as f64)
}

fn colored_time(time: u64) -> Colored<String> {
    let text = display_time(time);
    match time {
        _ if time < 10_000_000 => text.green(),
        _ if time < 100_000_000 => text.yellow(),
        _ => text.red(),
    }
}

pub fn stats(total_time: u64, num_parts: u8) {
    log("parts".normal(), format!("{num_parts:02}/49"));
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

struct Colored<T> {
    value: T,
    color: ColorSpec,
}

impl<T> Colored<T> {
    fn bold(mut self) -> Self {
        self.color.set_bold(true);
        self
    }
}

impl<T> From<T> for Colored<T>
where
    T: fmt::Display,
{
    fn from(value: T) -> Self {
        value.normal()
    }
}

trait Colorer<T> {
    fn normal(self) -> Colored<T>;
    fn green(self) -> Colored<T>;
    fn yellow(self) -> Colored<T>;
    fn red(self) -> Colored<T>;
    fn dim(self) -> Colored<T>;
}

impl<T> Colorer<T> for T {
    fn normal(self) -> Colored<T> {
        Colored {
            value: self,
            color: ColorSpec::new(),
        }
    }

    fn green(self) -> Colored<T> {
        let mut color = ColorSpec::new();
        color.set_fg(Some(Color::Green));
        Colored { value: self, color }
    }

    fn yellow(self) -> Colored<T> {
        let mut color = ColorSpec::new();
        color.set_fg(Some(Color::Yellow));
        Colored { value: self, color }
    }

    fn red(self) -> Colored<T> {
        let mut color = ColorSpec::new();
        color.set_fg(Some(Color::Red));
        Colored { value: self, color }
    }

    fn dim(self) -> Colored<T> {
        let mut color = ColorSpec::new();
        color.set_dimmed(true);
        Colored { value: self, color }
    }
}
