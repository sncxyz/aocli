use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::{
    display,
    error::{AocError, Context, Result},
    ROOT,
};

pub enum CurrentDirectory {
    Unknown,
    Root,
    Year { year: String },
    Day { year: String, day: String },
}

impl CurrentDirectory {
    pub fn get() -> Result<(PathBuf, Self)> {
        let current = env::current_dir().context("failed to get current directory")?;
        if current.join(ROOT).try_is_file()? {
            return Ok((current, Self::Root));
        }
        let Some(parent) = current.parent() else {
            return Ok((current, Self::Unknown));
        };
        if parent.join(ROOT).try_is_file()? {
            let year = year_from_dir(&current).ok_or(AocError::YearDir)?;
            return Ok((parent.into(), Self::Year { year }));
        }
        let Some(grandparent) = parent.parent() else {
            return Ok((current, Self::Unknown));
        };
        if grandparent.join(ROOT).try_is_file()? {
            let year = year_from_dir(parent).ok_or(AocError::YearDir)?;
            let day = day_from_dir(&current).ok_or(AocError::DayDir)?;
            return Ok((grandparent.into(), Self::Day { year, day }));
        }
        Ok((current, Self::Unknown))
    }
}

fn dir_name(path: &Path) -> Option<String> {
    Some(path.file_name()?.to_str()?.to_string())
}

fn year_from_dir(path: &Path) -> Option<String> {
    let year = dir_name(path)?;
    let num = year.parse::<u16>().ok()?;
    ((2015..10000).contains(&num) && num.to_string() == year).then_some(year)
}

fn day_from_dir(path: &Path) -> Option<String> {
    let day = dir_name(path)?;
    let num = day.parse::<u16>().ok()?;
    ((1..=25).contains(&num) && day.len() == 2).then_some(day)
}

pub trait PathInfo
where
    Self: AsRef<Path>,
{
    fn try_is_file(&self) -> Result<bool> {
        let path = self.as_ref();
        Ok(if path.try_exists().context(AocError::FileRead)? {
            path.is_file()
        } else {
            false
        })
    }

    fn try_is_dir(&self) -> Result<bool> {
        let path = self.as_ref();
        Ok(if path.try_exists().context(AocError::FileRead)? {
            path.is_dir()
        } else {
            false
        })
    }

    fn read_file(&self) -> Result<FileInfo> {
        Ok(FileInfo {
            path: self.as_ref(),
            contents: if self.try_is_file()? {
                let contents = fs::read_to_string(self).context(AocError::FileRead)?;
                let contents = contents.trim_end();
                if contents.is_empty() {
                    FileContents::Empty
                } else {
                    FileContents::Contents(contents.to_string())
                }
            } else {
                FileContents::Nonexistent
            },
        })
    }

    fn assert_year_dir(&self) -> Result<()> {
        self.try_is_dir()?
            .then_some(())
            .ok_or_else(|| AocError::MissingYearDir(display::path(self.as_ref())).into())
    }

    fn assert_day_dir(&self) -> Result<()> {
        self.try_is_dir()?
            .then_some(())
            .ok_or_else(|| AocError::MissingDayDir(display::path(self.as_ref())).into())
    }
}

impl<P: AsRef<Path>> PathInfo for P {}

pub struct FileInfo<'a> {
    path: &'a Path,
    contents: FileContents,
}

pub enum FileContents {
    Empty,
    Contents(String),
    Nonexistent,
}

impl<'a> FileInfo<'a> {
    pub fn try_contents(self) -> Result<String> {
        match self.contents {
            FileContents::Empty => Err(AocError::EmptyFile(display::path(self.path)).into()),
            FileContents::Nonexistent => Err(AocError::NoFile(display::path(self.path)).into()),
            FileContents::Contents(contents) => Ok(contents),
        }
    }

    pub fn get_contents(self) -> Option<String> {
        if let FileContents::Contents(contents) = self.contents {
            return Some(contents);
        }
        None
    }

    pub fn has_contents(&self) -> bool {
        matches!(self.contents, FileContents::Contents(_))
    }
}
