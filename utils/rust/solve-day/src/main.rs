use std::{fs::create_dir_all, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod input_downloader;
mod language_templates;
mod revert_changes;

use cli::{Cli, SolutionLang};
use input_downloader::cache_input;
use language_templates::Template;
use revert_changes::revert_changes;

fn main() -> Result<()> {
    if let Err(e) = dotenv::from_filename("Config.env") {
        if e.not_found() {
            return Err(e).context("Failed to load config file");
        } else {
            eprintln!("Couldn't read Config.env file");
            eprintln!("Continuing without it --");
        }
    }
    let args = Cli::parse();
    let day = args.dayarg.day;
    let year = args.dayarg.year;
    let lang = args.solution_lang;
    let key = args.session_key;
    let e = run(day, year, lang, key);
    if let Err(e) = e {
        if !args.no_revert_on_fail {
            eprintln!("Encountered an error during template generation!");
            eprintln!("Trying to revert changes");
            if let Err(err) = revert_changes(day, year, lang) {
                Err(e).context(err)
            } else {
                Err(e)
            }
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

fn run(day: u8, year: u16, lang: SolutionLang, key: String) -> Result<()> {
    let mut path = PathBuf::from(format!("{year}/{lang}"));
    create_dir_all(&path)?;
    path.push(format!("{day:02}"));
    lang.into_template().init_at_path(path, day, year)?;
    cache_input(day, year, &key)?;
    Ok(())
}

impl cli::SolutionLang {
    fn into_template(self) -> &'static dyn Template {
        match self {
            cli::SolutionLang::Rust => &language_templates::RustTemplate,
            cli::SolutionLang::Python => &language_templates::PythonTemplate,
        }
    }
}
