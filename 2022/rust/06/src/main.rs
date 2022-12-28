use anyhow::{bail, ensure, Result};
use aoc_parse::{parser, prelude::*};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/06"));

#[allow(unused)]
type Int = i64;
type Input = String;

fn parse(input: &'static str) -> Result<Input> {
    let p = parser!(string(any_char+));
    p.parse(input).map_err(Into::into)
}

fn part1(input: Input) -> Result<String> {
    // pad the input because the search loop does not check the starting substring
    let mut chars = std::iter::once((' ', 0)).chain(input.chars().zip(1..));
    let cur_window = [(); 4].map(|_| chars.next().map(|(c, _)| c));
    ensure!(cur_window.iter().all(Option::is_some));
    let mut cur_window = cur_window.map(Option::unwrap);
    for (c, i) in chars {
        cur_window[0] = c;
        cur_window.rotate_left(1);
        if cur_window
            .iter()
            .all(|c| cur_window.iter().filter(|&x| x == c).count() == 1)
        {
            return Ok(i.to_string());
        }
    }
    bail!("Given datastream does not contain any start-of-packet marker");
}

fn part2(input: Input) -> Result<String> {
    // pad the input because the search loop does not check the starting substring
    let mut chars = std::iter::once((' ', 0)).chain(input.chars().zip(1..));
    let cur_window = [(); 14].map(|_| chars.next().map(|(c, _)| c));
    ensure!(cur_window.iter().all(Option::is_some));
    let mut cur_window = cur_window.map(Option::unwrap);
    for (c, i) in chars {
        cur_window[0] = c;
        cur_window.rotate_left(1);
        if cur_window
            .iter()
            .all(|c| cur_window.iter().filter(|&x| x == c).count() == 1)
        {
            return Ok(i.to_string());
        }
    }
    bail!("Given datastream does not contain any start-of-message marker");
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
    use aoc_utils::TestCase;

    use super::*;

    const TEST_CASES: &[[&str; 3]] = &[
        ["mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", "19"],
        ["bvwbjplbgvbhsrlpgdmjqwftvncz", "5", "23"],
        ["nppdvjthqldpwncqszvftbrmjlhg", "6", "23"],
        ["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", "29"],
        ["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", "26"],
        ["zcfgfwzzqfrljwzlrfnpqdbhtmscgvjw", "4", "26"],
    ];

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
