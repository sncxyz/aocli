use crate::{error::Result, file::PathInfo};

use std::{fs, path::Path, process::Command};

pub fn build(path: &Path, debug: bool, show_output: bool) -> Result<BuildResult> {
    let mut command = Command::new("cargo");
    command.current_dir(path).arg("build");
    if !debug {
        command.arg("-r");
    }
    let status = if show_output {
        command.spawn()?.wait()?
    } else {
        command.output()?.status
    };
    Ok(if status.success() {
        BuildResult::Success
    } else {
        BuildResult::Failure
    })
}

pub enum BuildResult {
    Success,
    Failure,
}

impl BuildResult {
    pub fn success(&self) -> bool {
        matches!(self, Self::Success)
    }
}

pub fn run(
    path: &Path,
    day: &str,
    input: &str,
    part: &str,
    debug: bool,
    show_output: bool,
) -> Result<RunResult> {
    let exe = path
        .join("target")
        .join(if debug { "debug" } else { "release" })
        .join(format!("day-{day}"));
    let mut command = Command::new(exe);
    command.current_dir(path).arg(input).arg(part);
    let status = if show_output {
        command.spawn()?.wait()?
    } else {
        command.output()?.status
    };
    if !status.success() {
        return Ok(RunResult::Panic);
    }
    let out = path.join("data").join(input).join(part).join("out");
    if out.join("unimplemented").try_is_file()? {
        return Ok(RunResult::Unimplemented);
    }
    let answer = fs::read_to_string(out.join("answer"))?;
    let time = fs::read_to_string(out.join("time"))?.parse::<u64>()?;
    Ok(RunResult::Success { answer, time })
}

pub enum RunResult {
    Unimplemented,
    Panic,
    Success { answer: String, time: u64 },
}
