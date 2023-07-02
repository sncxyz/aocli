use crate::display;

use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    #[error("invalid directory for command `{0}`")]
    CommandDir(String),
    #[error("invalid year directory")]
    YearDir,
    #[error("invalid day directory")]
    DayDir,
    #[error("must be an integer at least 2015")]
    YearArg,
    #[error("must be an integer between 1 and 25")]
    DayArg,
    #[error("unexpected argument `{0}`")]
    ExtraArg(String),
    #[error("invalid value for argument <{0}>: `{1}`")]
    InvalidArg(Arg, String),
    #[error("missing argument <{0}>")]
    MissingArg(Arg),
    #[error("must be `1` or `2`")]
    Part,
    #[error("year directory not found: {0}")]
    MissingYearDir(String),
    #[error("day directory not found: {0}")]
    MissingDayDir(String),
    #[error("path already exists: {0}")]
    PathExists(String),
    #[error("failed to read file system")]
    FileSystemRead,
    #[error("failed to write to file system")]
    FileSystemWrite,
    #[error("failed to initialise git repository")]
    GitInit,
    #[error("file empty at {0}")]
    EmptyFile(String),
    #[error("no file found at {0}")]
    NoFile(String),
    #[error("no puzzle input")]
    NoInput,
    #[error("network error")]
    Network,
    #[error("invalid session cookie")]
    Session,
    #[error("server response error")]
    Response,
    #[error("webpage not available")]
    PageAvailable,
    #[error("failed to open browser")]
    Browser,
    #[error("no days to run")]
    NoDays,
    #[error("invalid term in argument <DAYS>: `{0}`")]
    InvalidTerm(String),
    #[error("term must be in the form X, -X, X..Y or -X..Y")]
    TermFormat,
    #[error("day must be between 1 and 25")]
    TermDayRange,
    #[error("invalid input name")]
    InputName,
    #[error("not a valid directory name")]
    InputNameFormat,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

pub struct Error {
    error: Vec<String>,
    usage: Vec<String>,
}

impl<E: ToString> From<E> for Error {
    fn from(value: E) -> Self {
        Self {
            error: vec![value.to_string()],
            usage: Vec::new(),
        }
    }
}

impl Error {
    pub fn context<C: ToString>(mut self, context: C) -> Self {
        self.error.push(context.to_string());
        self
    }

    pub fn with_context<C: ToString, F: Fn() -> C>(mut self, context: F) -> Self {
        self.error.push(context().to_string());
        self
    }

    pub fn usage<U: ToString>(mut self, usage: U) -> Self {
        self.usage.push(usage.to_string());
        self
    }

    pub fn with_usage<U: ToString, F: Fn() -> U>(mut self, usage: F) -> Self {
        self.usage.push(usage().to_string());
        self
    }

    pub fn usages<U: ToString>(mut self, usages: impl IntoIterator<Item = U>) -> Self {
        for usage in usages {
            self = self.usage(usage);
        }
        self
    }
}

pub trait Context<T, E: Into<Error>> {
    fn context<C: ToString>(self, context: C) -> Result<T, Error>;
    fn with_context<C: ToString, F: Fn() -> C>(self, context: F) -> Result<T, Error>;
    fn usage<U: ToString>(self, usage: U) -> Result<T, Error>;
    fn with_usage<U: ToString, F: Fn() -> U>(self, usage: F) -> Result<T, Error>;
    fn usages<U: ToString>(self, usages: impl IntoIterator<Item = U>) -> Result<T, Error>;
}

impl<T, E: Into<Error>> Context<T, E> for Result<T, E> {
    fn context<C: ToString>(self, context: C) -> Result<T, Error> {
        self.map_err(|e| e.into().context(context))
    }

    fn with_context<C: ToString, F: Fn() -> C>(self, context: F) -> Result<T, Error> {
        self.map_err(|e| e.into().with_context(context))
    }

    fn usage<U: ToString>(self, usage: U) -> Result<T, Error> {
        self.map_err(|e| e.into().usage(usage))
    }

    fn with_usage<U: ToString, F: Fn() -> U>(self, usage: F) -> Result<T, Error> {
        self.map_err(|e| e.into().with_usage(usage))
    }

    fn usages<U: ToString>(self, usages: impl IntoIterator<Item = U>) -> Result<T, Error> {
        self.map_err(|e| e.into().usages(usages))
    }
}

pub trait ErrorDisplayer {
    type Output;
    fn display_err(self) -> Option<Self::Output>;
}

impl<T, E: Into<Error>> ErrorDisplayer for Result<T, E> {
    type Output = T;

    fn display_err(self) -> Option<Self::Output> {
        match self {
            Ok(x) => Some(x),
            Err(e) => {
                let e: Error = e.into();
                e.display_err();
                None
            }
        }
    }
}

impl ErrorDisplayer for Error {
    type Output = ();

    fn display_err(self) -> Option<Self::Output> {
        let mut errors = self.error.into_iter().rev();
        display::error(errors.next().unwrap());
        for error in errors {
            display::cause(error);
        }
        let mut usages = self.usage.into_iter();
        if let Some(usage) = usages.next() {
            display::usage(usage);
            for usage in usages {
                display::or(usage);
            }
        }
        None
    }
}

pub trait ToErr {
    fn err<T>(self) -> Result<T>;
    fn error(self) -> Error;
}

impl<E: Into<Error>> ToErr for E {
    fn err<T>(self) -> Result<T> {
        Err(self.into())
    }

    fn error(self) -> Error {
        self.into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Arg {
    Year,
    Day,
    Part,
    Input,
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Year => "YEAR",
                Self::Day => "DAY",
                Self::Part => "PART",
                Self::Input => "INPUT",
            }
        )
    }
}
