use anyhow::{bail, Result};
use aoc_parse::{parser, prelude::*};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/05"));

#[allow(unused)]
type Int = i64;
type Input = (CrateStacks, Vec<CraneMove>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CraneMove {
    from: usize,
    to: usize,
    amt: i64,
}

impl CraneMove {
    fn new(amt: i64, from: usize, to: usize) -> Self {
        assert_ne!(
            from, to,
            "Crane should not transport crates from one stack to the same stack"
        );
        Self {
            from: from - 1,
            to: to - 1,
            amt,
        }
    }
}

#[derive(Debug, Clone)]
struct CrateStacks {
    stacks: Vec<Vec<char>>,
}

impl CrateStacks {
    fn get_stacks(&mut self, from: usize, to: usize) -> (&mut Vec<char>, &mut Vec<char>) {
        if from < to {
            let (from_part, to_part) = self.stacks.split_at_mut(to);
            (&mut from_part[from], &mut to_part[0])
        } else {
            let (to_part, from_part) = self.stacks.split_at_mut(from);
            (&mut from_part[0], &mut to_part[to])
        }
    }
}

fn parse(input: &'static str) -> Result<Input> {
    let crate_stack_level_parser = parser!(repeat_sep({'[' x:alpha ']' => x, "   " => ' '}, ' '));
    let crane_move_parser = parser!(lines("move " amt:i64 " from " from:usize " to " to:usize => CraneMove::new(amt, from, to)));

    let Some((crate_stack, moves_input)) = input.split_once("\n\n") else { bail!("There is no newline between"); };
    let Some((crate_stack, stack_count)) = crate_stack.rsplit_once('\n') else { bail!("The stacks seem to be empty"); };

    let stack_count = stack_count.split_whitespace().count();
    let mut crate_stacks = CrateStacks {
        stacks: vec![vec![]; stack_count],
    };

    for line in crate_stack.lines().rev() {
        crate_stack_level_parser
            .parse(line)?
            .into_iter()
            .zip(crate_stacks.stacks.iter_mut())
            .for_each(|(item, stack)| {
                if item != ' ' {
                    stack.push(item);
                }
            });
    }

    let moves_input = crane_move_parser.parse(moves_input)?;
    Ok((crate_stacks, moves_input))
}

fn part1((mut stacks, moves): Input) -> Result<String> {
    for CraneMove { amt, from, to } in moves {
        let remove_start = stacks.stacks[from].len() - amt as usize;
        let (from_stack, to_stack) = stacks.get_stacks(from, to);
        to_stack.extend(from_stack.drain(remove_start..).rev());
    }
    Ok(stacks
        .stacks
        .iter()
        .flat_map(|s| s.last().copied())
        .collect())
}

fn part2((mut stacks, moves): Input) -> Result<String> {
    for CraneMove { amt, from, to } in moves {
        let remove_start = stacks.stacks[from].len() - amt as usize;
        let (from_stack, to_stack) = stacks.get_stacks(from, to);
        to_stack.extend(from_stack.drain(remove_start..));
    }
    Ok(stacks
        .stacks
        .iter()
        .flat_map(|s| s.last().copied())
        .collect())
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
                [D]    
            [N] [C]    
            [Z] [M] [P]
            1   2   3 
            
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "},
        "CMZ",
        "MCD",
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
