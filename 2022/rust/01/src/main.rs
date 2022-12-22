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
}
