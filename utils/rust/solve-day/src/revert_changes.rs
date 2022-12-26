use std::{fs, process::Command};

use anyhow::{Context, Ok, Result};

use crate::cli::SolutionLang;

pub fn revert_changes(day: u8, year: u16, lang: SolutionLang) -> Result<()> {
    if let Err(e) = revert_with_vcs(day, year, lang) {
        eprintln!("Revertion using vcs failed");
        eprintln!("Trying to revert manually");
        revert_manually(day, year, lang).context(e)
    } else {
        Ok(())
    }
}

fn revert_with_vcs(day: u8, year: u16, lang: SolutionLang) -> Result<()> {
    eprintln!("Trying to revert using git");
    let changed_files = Command::new("git")
        .args(["status", "--porcelain"])
        .output()?;
    let changed_files: String = String::from_utf8(changed_files.stdout)?;
    let solution_dir = format!("{year}/{lang}/{day:02}/");
    let input_file = format!("{year}/inputs/{day:02}");
    let changed_only_solution_files = changed_files
        .lines()
        .map(|l| {
            l.trim()
                .split_once(' ')
                .expect("git should not return empty lines")
                .1
                .trim()
        })
        .all(|file| file.starts_with(&solution_dir) || file == input_file);
    if changed_only_solution_files {
        Command::new("git").args(["clean", "-f"]).spawn()?.wait()?;
        Command::new("git")
            .args(["checkout", "."])
            .spawn()?
            .wait()?;
    }
    Ok(())
}

fn revert_manually(day: u8, year: u16, lang: SolutionLang) -> Result<()> {
    fs::remove_file(format!("{year}/{lang}/inputs/{day:02}"))?;
    fs::remove_dir_all(format!("{year}/{lang}/{day:02}"))?;
    Ok(())
}
