use aocli::error::{AocError, Context, Result, ToErr};

use regex::Regex;

pub fn get_input(year: &str, day: &str, session: &str) -> Result<String> {
    let day = &day.parse::<u8>().unwrap().to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = ureq::get(&url).set("cookie", session).call();
    match response {
        Ok(response) => {
            let text = response.into_string().context(AocError::Response)?;
            Ok(text.trim_end().to_string())
        }
        Err(e) => Err(match e {
            ureq::Error::Status(_, response) => {
                let text = response.into_string().context(AocError::Response)?;
                let re = Regex::new(r"Puzzle inputs differ by user").unwrap();
                if re.is_match(&text) {
                    AocError::Session
                } else {
                    AocError::DayAvailable
                }
                .into()
            }
            ureq::Error::Transport(transport) => {
                transport.to_string().error().context(AocError::Network)
            }
        }),
    }
}

pub struct Progress {
    pub part_1: Option<String>,
    pub part_2: Option<String>,
    pub next: Option<String>,
}

pub fn get_progress(year: &str, day: &str, session: &str) -> Result<Progress> {
    let day = &day.parse::<u8>().unwrap().to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    let response = ureq::get(&url).set("cookie", session).call();
    let text = match response {
        Ok(response) => {
            let text = response.into_string().context(AocError::Response)?;
            let re =
                Regex::new(r"To play, please identify yourself via one of these services").unwrap();
            if re.is_match(&text) {
                return AocError::Session.err();
            } else {
                text
            }
        }
        Err(e) => {
            return match e {
                ureq::Error::Status(_, _) => AocError::DayAvailable.err(),
                ureq::Error::Transport(transport) => {
                    transport.to_string().err().context(AocError::Network)
                }
            }
        }
    };
    let re = Regex::new(r"Your puzzle answer was <code>([^<]+)</code>").unwrap();
    let caps: Vec<_> = re.captures_iter(&text).take(2).collect();
    let (part_1, part_2) = match caps.len() {
        0 => (None, None),
        1 => (Some(caps[0][1].to_string()), None),
        2 => (Some(caps[0][1].to_string()), Some(caps[1][1].to_string())),
        _ => unreachable!(),
    };
    let next = (caps.len() <= (day != "25") as usize).then_some((caps.len() as u8 + 1).to_string());
    Ok(Progress {
        part_1,
        part_2,
        next,
    })
}

pub enum SubmissionResult {
    Correct,
    Incorrect,
    Wait,
}

pub fn submit(
    year: &str,
    day: &str,
    part: &str,
    answer: &str,
    session: &str,
) -> Result<SubmissionResult> {
    let day = &day.parse::<u8>().unwrap().to_string();
    let url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let params = &[("level", part), ("answer", answer)];
    let response = ureq::post(&url).set("cookie", session).send_form(params);
    match response {
        Ok(response) => {
            let text = response.into_string().context(AocError::Response)?;
            let re = Regex::new(r"That's the right answer").unwrap();
            if re.is_match(&text) {
                return Ok(SubmissionResult::Correct);
            }
            let re = Regex::new(r"That's not the right answer").unwrap();
            if re.is_match(&text) {
                return Ok(SubmissionResult::Incorrect);
            }
            let re = Regex::new(r"You gave an answer too recently").unwrap();
            if re.is_match(&text) {
                return Ok(SubmissionResult::Wait);
            }
            AocError::Response.err()
        }
        Err(e) => match e {
            ureq::Error::Status(_, _) => AocError::DayAvailable.err(),
            ureq::Error::Transport(transport) => {
                transport.to_string().err().context(AocError::Network)
            }
        },
    }
}
