use std::ops::RangeInclusive;

use anyhow::Result;

use aoc_parse::{parser, prelude::*};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/04"));

type Input = Vec<(RangeInclusive<i64>, RangeInclusive<i64>)>;

fn parse(input: &'static str) -> Result<Input> {
    let range_parser = parser!(start:i64 '-' end:i64 => start..=end);
    let p = parser!(lines(range_parser ',' range_parser));
    let pairs = p.parse(input);
    pairs.map_err(Into::into)
}

fn part1(input: Input) -> Result<String> {
    Ok(input
        .into_iter()
        .filter(|(elf1, elf2)| {
            (elf1.start() <= elf2.start() && elf1.end() >= elf2.end())
                || (elf1.start() >= elf2.start() && elf1.end() <= elf2.end())
        })
        .count()
        .to_string())
}

fn part2(input: Input) -> Result<String> {
    Ok(input
        .into_iter()
        .filter(|(elf1, elf2)| {
            elf1.contains(elf2.start())
                || elf1.contains(elf2.end())
                || elf2.contains(elf1.start())
                || elf2.contains(elf1.end())
        })
        .count()
        .to_string())
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

    const TEST_CASES: &[[&str; 3]] = &[[
        indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "},
        "2",
        "4",
    ]];

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
