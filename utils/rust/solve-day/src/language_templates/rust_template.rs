use indoc::indoc;
use std::fmt::Write;
use std::path::PathBuf;

static MAIN_RS: &str = indoc!(
    r#"
    use anyhow::Result;

    static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/DAY"));

    #[allow(unused)]
    type Int = i64;
    type Input = String;

    fn parse(input: &'static str) -> Result<Input> {
        Ok(input.to_string())
    }
    
    fn part1(input: Input) -> Result<String> {
        todo!()
    }

    fn part2(input: Input) -> Result<String> {
        todo!()
    }

    fn main() -> Result<()> {
        let input = parse(INPUT)?;
        let part1 = part1(input.clone())?;
        println!("part1: {part1}");
        let part2 = part2(input)?;
        println!("part2: {part2}");
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use indoc::indoc;
    
        use aoc_utils::TestCase;
    
        use super::*;
    
        const TEST_CASES: &[[&str; 3]] = &[[indoc!(""), "", ""]];
    
        #[test]
        fn part1_test() {
            for case in TEST_CASES {
                let case = TestCase::from(case);
                let input = parse(case.input).unwrap();
                let output = part1(input).unwrap();
                assert_eq!(output, case.out1, "Failed for {case:?}");
            }
        }

        #[test]
        fn part2_test() {
            for case in TEST_CASES {
                let case = TestCase::from(case);
                let input = parse(case.input).unwrap();
                let output = part2(input).unwrap();
                assert_eq!(output, case.out2, "Failed for {case:?}");
            }
        }
    }
    "#
);

static CARGO_TOML: &str = indoc!(
    r#"
    [package]
    name = "aoc-YEAR-DAY"
    version = "0.1.0"
    edition = "2021"

    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

    [dependencies]
    anyhow.workspace = true
    date_time.workspace = true
    indoc.workspace = true
    aoc-parse.workspace = true
    regex.workspace = true

    aoc-utils = { path = "../../../utils/rust/aoc-utils" }
    "#
);

pub struct RustTemplate;

impl super::Template for RustTemplate {
    fn init_at_path(&self, mut path: PathBuf, day: u8, year: u16) -> std::io::Result<()> {
        let main_rs = format_main_rs(day);
        let cargo_toml = format_cargo_toml(day, year);

        path.push("src");
        std::fs::create_dir_all(&path)?;
        path.push("main.rs");
        std::fs::write(&path, &main_rs as &str)?;
        path.pop();
        path.pop();
        path.push("Cargo.toml");
        std::fs::write(&path, cargo_toml)?;
        Ok(())
    }
}

fn format_main_rs(day: u8) -> String {
    let mut main_rs = String::with_capacity(MAIN_RS.len() - 1);
    let placeholder_pos = MAIN_RS.find("DAY").unwrap();
    write!(&mut main_rs, "{}", &MAIN_RS[..placeholder_pos]).unwrap();
    write!(&mut main_rs, "{day:02}").unwrap();
    write!(&mut main_rs, "{}", &MAIN_RS[placeholder_pos + 3..]).unwrap();
    main_rs
}

fn format_cargo_toml(day: u8, year: u16) -> String {
    let mut cargo_toml = String::with_capacity(CARGO_TOML.len() - 1);
    let placeholder_pos = CARGO_TOML.find("YEAR-DAY").unwrap();
    write!(&mut cargo_toml, "{}", &CARGO_TOML[..placeholder_pos]).unwrap();
    write!(&mut cargo_toml, "{year}-{day:02}").unwrap();
    write!(&mut cargo_toml, "{}", &CARGO_TOML[placeholder_pos + 8..]).unwrap();
    cargo_toml
}
