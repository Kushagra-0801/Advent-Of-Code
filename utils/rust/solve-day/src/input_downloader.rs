use std::{fs, path::PathBuf};

use anyhow::Ok;

pub fn cache_input(day: u8, year: u16, session_cookie: &str) -> anyhow::Result<()> {
    let mut path = PathBuf::from(format!("{year}/inputs/{day:02}"));
    if path.is_file() {
        return Ok(());
    }
    path.pop();
    fs::create_dir_all(&path)?;
    let input = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={session_cookie}"));
    let input = input.call()?.into_string()?;
    path.push(format!("{day:02}"));
    fs::write(path, input)?;
    Ok(())
}
