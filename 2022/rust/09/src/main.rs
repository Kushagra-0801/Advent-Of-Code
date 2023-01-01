use std::{collections::HashSet, ops::Add};

use anyhow::Result;
use aoc_parse::{parser, prelude::*};
use aoc_utils::indices::{IdxDelta, Sidx};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/09"));

#[allow(unused)]
type Int = i64;
type Input = Vec<Move>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    dir: Dir,
    mag: usize,
}

fn parse(input: &'static str) -> Result<Input> {
    use Dir::*;
    let p = parser!(lines(dir:{'L' => Left, 'R' => Right, 'U' => Up, 'D' => Down} ' ' mag:usize => Move {dir, mag}));
    p.parse(input).map_err(Into::into)
}

impl Move {
    fn into_iter(self) -> impl Iterator<Item = IdxDelta> {
        let delta = match self.dir {
            Dir::Left => IdxDelta { row: 0, col: -1 },
            Dir::Right => IdxDelta { row: 0, col: 1 },
            Dir::Up => IdxDelta { row: -1, col: 0 },
            Dir::Down => IdxDelta { row: 1, col: 0 },
        };
        (0..self.mag).map(move |_| delta)
    }
}

#[derive(Debug, Clone)]
struct Rope {
    head: Sidx,
    tail: Sidx,
}

impl Add<IdxDelta> for Rope {
    type Output = Self;

    fn add(self, rhs: IdxDelta) -> Self::Output {
        let head = self.head + rhs;
        let diff = head - self.tail;
        let tail = if diff.row.abs() > 1 || diff.col.abs() > 1 {
            self.head
        } else {
            self.tail
        };
        Self { head, tail }
    }
}

fn part1(input: Input) -> Result<String> {
    let mut rope = Rope {
        head: Sidx { row: 0, col: 0 },
        tail: Sidx { row: 0, col: 0 },
    };
    let moves = input.into_iter().flat_map(Move::into_iter);
    let mut visited = HashSet::new();
    visited.insert(rope.tail);
    for m in moves {
        rope = rope + m;
        visited.insert(rope.tail);
    }
    Ok(visited.len().to_string())
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

    const TEST_CASES: &[[&str; 3]] = &[[
        indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "},
        "13",
        "",
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
