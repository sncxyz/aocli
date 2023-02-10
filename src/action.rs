use crate::{network, run, Parts};

use aocli::{
    display,
    error::{AocError, Context, ErrorDisplayer, Result, ToErr},
    file::{FileInfo, PathInfo},
    ROOT,
};

use std::{fs, path::Path, process};

pub fn init(root: &Path) -> Result<()> {
    write_project_file(ROOT, root, "")?;
    write_project_file(
        ".gitignore",
        root,
        "target/\n!**/data/target/\nCargo.lock\n/.session\n**/[1-2]/out/",
    )?;
    write_project_file(".session", root, "")?;
    write_project_file(
        "README.md",
        root,
        "Solutions to the puzzles at \
    [Advent of Code](https://adventofcode.com) using \
    [aocli](https://github.com/scjqt/aocli).",
    )?;
    let output = process::Command::new("git")
        .arg("init")
        .current_dir(root)
        .output()
        .context(AocError::GitInit);
    if let Some(output) = output.display_err() {
        if !output.status.success() {
            String::from_utf8_lossy(&output.stderr)
                .error()
                .context(AocError::GitInit)
                .display_err();
        }
    }
    Ok(())
}

fn write_project_file(name: &str, root: &Path, contents: &str) -> Result<()> {
    let path = &root.join(name);
    if path.try_is_file()? {
        display::info!("file `{name}` already exists");
    } else {
        fs::write(path, contents).context(AocError::FileSystemWrite)?;
        display::success!("wrote file `{name}`");
    }
    Ok(())
}

pub fn new_day(path: &Path, year: &str, day: &str) -> Result<()> {
    if path.try_exists().context(AocError::FileSystemRead)? {
        return AocError::PathExists(display::path(path)).err();
    }
    write_day_files(path, day).context(AocError::FileSystemWrite)?;
    display::success!("created crate for {year}/{day}");
    display::info!("building crate...");
    if run::build(path, false, false).display_err().is_some() {
        display::success!("finished building crate");
    }
    Ok(())
}

fn write_day_files(path: &Path, day: &str) -> Result<()> {
    let is_day_25 = day == "25";
    fs::create_dir_all(path.join("src"))?;
    fs::write(
        path.join("Cargo.toml"),
        format!(
            "[package]\n\
        name = \"day-{day}\"\n\
        version = \"0.1.0\"\n\
        edition = \"2021\"\n\n\
        [dependencies]\n\
        aocli-runner = {{ git = \"https://github.com/scjqt/aocli-runner\" }}"
        ),
    )?;
    if is_day_25 {
        fs::write(
            path.join("src").join("main.rs"),
            "aoc::parts!(1);\n\n\
        fn part_1(input: &[&str]) -> bool {\n    unimplemented!()\n}",
        )?;
    } else {
        fs::write(
            path.join("src").join("main.rs"),
            "aoc::parts!(1);\n\n\
        fn part_1(input: &[&str]) -> bool {\n    unimplemented!()\n}\n\n\
        // fn part_2(input: &[&str]) -> bool {\n//     unimplemented!()\n// }",
        )?;
    }
    let data = path.join("data").join("actual");
    fs::create_dir_all(&data)?;
    fs::write(data.join("input"), "")?;
    fs::create_dir(data.join("1"))?;
    fs::write(data.join("1").join("answer"), "")?;
    // if !is_day_25 {
    fs::create_dir(data.join("2"))?;
    fs::write(data.join("2").join("answer"), "")?;
    // }
    Ok(())
}

pub fn add_input(path: &Path, data: &str) -> Result<()> {
    let mut data_path = path.join("data");
    if !data_path.try_is_dir()? {
        fs::create_dir(&data_path).context(AocError::FileSystemWrite)?;
    }
    data_path.push(data);
    let data_path = &data_path;
    if data_path.try_is_dir()? {
        return AocError::PathExists(display::path(data_path)).err();
    }
    write_data_files(data_path).context(AocError::FileSystemWrite)?;
    display::success!("created input `{}` at {}", data, display::path(data_path));
    Ok(())
}

fn write_data_files(path: &Path) -> Result<()> {
    fs::create_dir(path)?;
    fs::write(path.join("input"), "")?;
    fs::create_dir(path.join("1"))?;
    fs::write(path.join("1").join("answer"), "")?;
    fs::create_dir(path.join("2"))?;
    fs::write(path.join("2").join("answer"), "")?;
    Ok(())
}

pub fn session(root: &Path) -> Result<()> {
    let browsers = bench_scraper::find_cookies()
        .map_err(|e| format!("{e:?}").error())
        .context("failed to get browser cookies")?;
    for browser_cookies in browsers {
        let browser = &browser_cookies.browser;
        for cookie in browser_cookies.cookies {
            if cookie.host.contains("adventofcode.com") && cookie.name == "session" {
                display::success!("found session cookie in {:?}", browser);
                let session = &cookie.value;
                fs::write(root.join(".session"), session).context(AocError::FileSystemWrite)?;
                return Ok(());
            }
        }
    }
    "failed to find session cookie in browser cookies".err()
}

pub fn run_day(
    path: &Path,
    year: &str,
    day: &str,
    input: &str,
    parts: Parts,
    debug: bool,
) -> Result<()> {
    let data_path = &path.join("data").join(input);
    data_path
        .join("input")
        .read_file()?
        .try_contents()
        .context(AocError::NoInput)?;
    if !run::build(path, debug, true)?.success() {
        return Ok(());
    }
    match &parts {
        Parts::Default => {
            let mut both_unimplemented = true;
            for part in ["1", "2"] {
                match run::run(path, day, input, part, debug, true)? {
                    run::RunResult::Success { answer, time } => {
                        let correct = get_correct(data_path, part)?;
                        both_unimplemented = false;
                        display::answer_full(year, day, part, &answer, correct.as_deref(), time);
                    }
                    run::RunResult::Panic => {
                        both_unimplemented = false;
                        display::day_part(year, day, part);
                        display::panic();
                    }
                    _ => (),
                }
            }
            if both_unimplemented {
                display::info!("both parts unimplemented");
            }
        }
        Parts::Part(part) => match run::run(path, day, input, part, debug, true)? {
            run::RunResult::Success { answer, time } => {
                let correct = get_correct(data_path, part)?;
                display::answer_full(year, day, part, &answer, correct.as_deref(), time);
            }
            run::RunResult::Unimplemented => {
                display::day_part(year, day, part);
                display::unimplemented();
            }
            run::RunResult::Panic => {
                display::day_part(year, day, part);
                display::panic();
            }
        },
    }
    Ok(())
}

pub fn run_year(path: &Path, year: &str) -> Result<()> {
    let mut total_time = 0;
    let mut num_parts = 0;
    for day_number in 1..=25 {
        let day = &format!("{day_number:02}");
        let path = &path.join(day);
        if !path.try_is_dir()? {
            continue;
        }
        display::day(year, day);
        if !path
            .join("data")
            .join("actual")
            .join("input")
            .read_file()?
            .has_contents()
        {
            display::no_input();
            continue;
        }
        if !run::build(path, false, false)?.success() {
            display::build_error();
            continue;
        }
        display::part("1");
        run_part(path, day, "1", &mut total_time, &mut num_parts)?;
        if day_number < 25 {
            display::day_part(year, day, "2");
            run_part(path, day, "2", &mut total_time, &mut num_parts)?;
        }
    }
    display::stats(total_time, num_parts);
    Ok(())
}

fn run_part(
    path: &Path,
    day: &str,
    part: &str,
    total_time: &mut u64,
    num_parts: &mut u8,
) -> Result<()> {
    match run::run(path, day, "actual", part, false, false)? {
        run::RunResult::Panic => display::panic(),
        run::RunResult::Unimplemented => display::unimplemented(),
        run::RunResult::Success { answer, time } => {
            let correct = get_correct(&path.join("data").join("actual"), part)?;
            display::answer(&answer, correct.as_deref(), time);
            println!();
            *total_time += time;
            *num_parts += 1;
        }
    };
    Ok(())
}

pub fn test_day(path: &Path, year: &str, day: &str, parts: Parts) -> Result<()> {
    if !run::build(path, false, true)?.success() {
        return Ok(());
    }
    let parts = &match parts {
        Parts::Default => vec!["1", "2"],
        Parts::Part(ref part) => vec![part.as_ref()],
    }[..];
    if test_parts(path, year, day, parts)? {
        display::info!("nothing to test");
    }
    Ok(())
}

fn test_parts(path: &Path, year: &str, day: &str, parts: &[&str]) -> Result<bool> {
    let mut implemented = [true, true];
    let mut empty = true;
    for dir in path
        .join("data")
        .read_dir()
        .context(AocError::FileSystemRead)?
    {
        let dir = &dir.context(AocError::FileSystemRead)?;
        let input = dir.file_name();
        let Some(input) = input.to_str() else {
            continue;
        };
        let data_path = &dir.path();
        if !data_path.join("input").read_file()?.has_contents() {
            continue;
        }
        for (i, &part) in parts.iter().enumerate() {
            if !implemented[i] {
                continue;
            }
            let correct = get_correct(data_path, part)?;
            if correct.is_none() {
                continue;
            }
            empty = false;
            display::day_part(year, day, part);
            match run::run(path, day, input, part, false, false)? {
                run::RunResult::Panic => display::panic_input(input),
                run::RunResult::Unimplemented => {
                    display::unimplemented();
                    implemented[i] = false;
                }
                run::RunResult::Success { answer, time } => {
                    display::answer(&answer, correct.as_deref(), time);
                    println!("  ({input})");
                }
            }
        }
    }
    Ok(empty)
}

pub fn test_year(path: &Path, year: &str) -> Result<()> {
    let mut empty = true;
    for day_number in 1..=25 {
        let day = &format!("{day_number:02}");
        let path = &path.join(day);
        if !path.try_is_dir()? || !run::build(path, false, false)?.success() {
            continue;
        }
        if !test_parts(path, year, day, &["1", "2"])? {
            empty = false;
        }
    }
    if empty {
        display::info!("nothing to test");
    }
    Ok(())
}

pub fn get(path: &Path, year: &str, day: &str) -> Result<()> {
    let parts = ["1", "2"];
    let data_path = &path.join("data").join("actual");
    let input_path = &data_path.join("input");
    let answer_paths = &parts.map(|part| data_path.join(part).join("answer"));

    let update_input = !input_path.read_file()?.has_contents();
    let update_answers = [
        !answer_paths[0].read_file()?.has_contents(),
        day != "25" && !answer_paths[1].read_file()?.has_contents(),
    ];

    if update_input || update_answers[0] || update_answers[1] {
        let session = &get_session(path.parent().unwrap().parent().unwrap())?;
        if !data_path.try_is_dir()? {
            fs::create_dir_all(data_path).context(AocError::FileSystemWrite)?;
        }
        if update_input {
            display::info!("downloading puzzle input...");
            let input = network::get_input(year, day, session)?;
            fs::write(input_path, input).context(AocError::FileSystemWrite)?;
            display::success!("input file written to {}", display::path(input_path));
        }
        if update_answers[0] || update_answers[1] {
            display::info!("downloading puzzle answers...");
            let progress = network::get_progress(year, day, session)?;
            let answers = [progress.part_1, progress.part_2];
            for i in 0..2 {
                if !update_answers[i] {
                    continue;
                }
                let part = parts[i];
                if let Some(answer) = &answers[i] {
                    let part_path = answer_paths[i].parent().unwrap();
                    if !part_path.try_is_dir()? {
                        fs::create_dir(part_path).context(AocError::FileSystemWrite)?;
                    }
                    fs::write(&answer_paths[i], answer).context(AocError::FileSystemWrite)?;
                    display::success!(
                        "answer to part {part} written to {}",
                        display::path(&answer_paths[i])
                    );
                } else {
                    display::info!("no answer to part {part} found");
                }
            }
        }
    } else {
        display::info!("nothing to update");
    }

    Ok(())
}

pub fn submit(path: &Path, year: &str, day: &str, answer: Option<&str>) -> Result<()> {
    let session = &get_session(path.parent().unwrap().parent().unwrap())?;
    display::info!("getting progress");
    let progress = network::get_progress(year, day, session)?;
    if let Some(part) = &progress.next {
        let answer_path = &path.join("data").join("actual").join(part);
        let answer = &if let Some(answer) = answer {
            answer.to_string()
        } else {
            answer_path
                .join("out")
                .join("answer")
                .read_file()?
                .try_contents()
                .map_err(|_| "no answer to submit")?
        };
        display::day_part(year, day, part);
        if !answer_path.try_is_dir()? {
            fs::create_dir_all(answer_path).context(AocError::FileSystemWrite)?;
        }
        let result = network::submit(year, day, part, answer, session);
        if result.is_err() {
            display::submit_error();
        }
        match result? {
            network::SubmissionResult::Correct => {
                display::just_answer(answer, true);
                fs::write(answer_path.join("answer"), answer).context(AocError::FileSystemWrite)?;
            }
            network::SubmissionResult::Wait => {
                display::wait();
            }
            network::SubmissionResult::Incorrect => {
                display::just_answer(answer, false);
            }
        }
    } else {
        display::info!("no part left to submit");
    }
    Ok(())
}

pub fn open_year(year: &str) -> Result<()> {
    webbrowser::open(&format!("https://adventofcode.com/{year}")).context(AocError::Browser)
}

pub fn open_day(year: &str, day: &str) -> Result<()> {
    let day = &day.parse::<u8>().unwrap().to_string();
    webbrowser::open(&format!("https://adventofcode.com/{year}/day/{day}"))
        .context(AocError::Browser)
}

pub fn help() -> Result<()> {
    webbrowser::open("https://github.com/scjqt/aocli/blob/master/README.md#commands")
        .context(AocError::Browser)
}

fn get_session(root: &Path) -> Result<String> {
    root.join(".session")
        .read_file()
        .and_then(FileInfo::try_contents)
        .map(|contents| format!("session={}", contents.trim()))
        .context("failed to get session cookie")
}

fn get_correct(data_path: &Path, part: &str) -> Result<Option<String>> {
    Ok(data_path
        .join(part)
        .join("answer")
        .read_file()?
        .get_contents())
}
