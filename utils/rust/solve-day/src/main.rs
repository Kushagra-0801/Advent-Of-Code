use std::{fs::create_dir_all, path::PathBuf};

use anyhow::Result;
use clap::Parser;

mod cli;

use cli::Cli;
use language_templates::Template;

fn main() -> Result<()> {
    let args = Cli::parse();
    let year = args.dayarg.year;
    let lang = args.solution_lang;
    let mut path = PathBuf::from(format!("{year}/{lang}"));
    match create_dir_all(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        e => e?,
    }
    let day = args.dayarg.day;
    path.push(format!("{day:02}"));
    lang.into_template().init_at_path(path, day, year)?;
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

mod language_templates;
