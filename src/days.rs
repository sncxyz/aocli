use crate::error::{AocError, Context, Result, ToErr};

pub fn parse_days(args: &[&str]) -> Result<Vec<u8>> {
    let mut terms = Vec::with_capacity(args.len());
    for &arg in args {
        terms.push(Term::parse(arg).context(AocError::InvalidTerm(arg.into()))?);
    }
    if terms.is_empty() {
        return "missing argument <DAYS>".err();
    }
    let mut days = [!terms[0].positive; 26];
    for term in terms {
        match term.days {
            Days::Day { day } => days[day as usize] = term.positive,
            Days::Range { from, to } => {
                for day in from..=to {
                    days[day as usize] = term.positive;
                }
            }
        }
    }
    Ok((1..=25).filter(|day| days[*day as usize]).collect())
}

struct Term {
    positive: bool,
    days: Days,
}

impl Term {
    fn parse(arg: &str) -> Result<Self> {
        if arg.is_empty() {
            return AocError::TermFormat.err();
        }
        Ok(if &arg[0..1] == "-" {
            Self {
                positive: false,
                days: Days::parse(&arg[1..])?,
            }
        } else {
            Self {
                positive: true,
                days: Days::parse(arg)?,
            }
        })
    }
}

enum Days {
    Day { day: u8 },
    Range { from: u8, to: u8 },
}

impl Days {
    fn parse(arg: &str) -> Result<Self> {
        let parts: Vec<_> = arg.split("..").collect();
        Ok(match parts.len() {
            1 => {
                let day = arg.parse().map_err(|_| AocError::TermFormat)?;
                if !(1..=25).contains(&day) {
                    return AocError::TermDayRange.err();
                }
                Self::Day { day }
            }
            2 => {
                let from = if parts[0].is_empty() {
                    1
                } else {
                    parts[0].parse().map_err(|_| AocError::TermFormat)?
                };
                let to = if parts[1].is_empty() {
                    25
                } else {
                    parts[1].parse().map_err(|_| AocError::TermFormat)?
                };
                if !(1..=25).contains(&from) || !(1..=25).contains(&to) {
                    return AocError::TermDayRange.err();
                }
                if from > to {
                    return "start of range is after end of range".err();
                }
                Self::Range { from, to }
            }
            _ => return AocError::TermFormat.err(),
        })
    }
}
