use std::{fs, path::PathBuf};

use anyhow::Ok;

pub fn cache_input(day: u8, year: u16, session_cookie: &str) -> anyhow::Result<()> {
    let input = ureq::get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .set("Cookie", &format!("session={session_cookie}"))
        .call()?
        .into_string()?;
    let path = PathBuf::from(format!("{year}/inputs/{day:02}"));
    fs::write(path, input)?;
    Ok(())
}
