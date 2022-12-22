use indoc::indoc;
use regex::{Captures, Regex};
use std::path::PathBuf;

static MAIN_RS: &str = indoc!(
    r#"
    use anyhow::Result;

    static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../input/{day:02}"));

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
        let part2 = part2(input.clone())?;
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
            Ok(())
        }

        #[test]
        fn part2_test() {
            for case in TEST_CASES {
                let case = TestCase::from(case);
                let input = parse(case.input).unwrap();
                let output = part2(input).unwrap();
                assert_eq!(output, case.out2, "Failed for {case:?}");
            }
            Ok(())
        }
    }
    "#
);

static CARGO_TOML: &str = indoc!(
    r#"
    [package]
    name = "aoc-{year}-{day:02}"
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
        let placeholder_pattern = Regex::new(r#"[{\w+}]"#).unwrap();
        let main_rs = placeholder_pattern.replace_all(MAIN_RS, |caps: &Captures| match &caps[1] {
            "day" => format!("{day:02}"),
            "year" => format!("{year}"),
            _ => todo!(),
        });
        path.push("src");
        std::fs::create_dir_all(&path)?;
        path.push("main.rs");
        std::fs::write(&path, &main_rs as &str)?;
        path.pop();
        path.pop();
        path.push("Cargo.toml");
        std::fs::write(&path, CARGO_TOML)?;
        Ok(())
    }
}

// fn format_day(day: u8) -> [u8; 2] {
//     if day < 10 {
//         [b'0', day - b'0']
//     } else {
//         [(day / 10) - b'0', (day % 10) - b'0']
//     }
// }
