use anyhow::{bail, Ok, Result};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/02"));

#[allow(unused)]
type Int = i64;
type Input = Vec<(RockPaperScissors, char)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockPaperScissors {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RockPaperScissors {
    fn play_against(self, other: Self) -> GameResult {
        use GameResult::*;
        if self == other {
            Draw
        } else if self.weak_against() == other {
            Lose
        } else {
            Win
        }
    }

    fn weak_against(&self) -> Self {
        use RockPaperScissors::*;
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn response_for_result(self, result: GameResult) -> Self {
        use GameResult::*;
        match result {
            Lose => self.weak_against().weak_against(),
            Draw => self,
            Win => self.weak_against(),
        }
    }
}

impl TryFrom<char> for RockPaperScissors {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let play = match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => bail!("Not a rock paper scissors move: {value:?}"),
        };
        Ok(play)
    }
}

fn parse(input: &'static str) -> Result<Input> {
    let mut moves = vec![];
    for strategy_line in input.lines() {
        let Some((opponent, response)) = strategy_line.split_once(' ') else { bail!("Invalid strategy: {strategy_line:?}") };
        let opponent = if opponent.len() == 1 {
            opponent.chars().next().expect("Checked the length above")
        } else {
            bail!("Invalid opponent move: {opponent:?}")
        };
        let response = if response.len() == 1 {
            response.chars().next().expect("Checked the length above")
        } else {
            bail!("Invalid response move: {response:?}")
        };
        let opponent = opponent.try_into()?;
        moves.push((opponent, response))
    }
    Ok(moves)
}

fn part1(input: Input) -> Result<String> {
    let parse_response = |res: char| -> Result<RockPaperScissors> {
        let res = match res {
            'X' => RockPaperScissors::Rock,
            'Y' => RockPaperScissors::Paper,
            'Z' => RockPaperScissors::Scissors,
            _ => bail!("Not a rock paper scissors move: {res:?}"),
        };
        Ok(res)
    };
    input
        .into_iter()
        .map(|(opp, res)| {
            parse_response(res)
                .map(|res| (opp, res))
                .map(|(o, r)| r as Int + r.play_against(o) as Int)
        })
        .sum::<Result<Int>>()
        .map(|x| x.to_string())
}

fn part2(input: Input) -> Result<String> {
    let parse_response = |res: char| -> Result<GameResult> {
        let res = match res {
            'X' => GameResult::Lose,
            'Y' => GameResult::Draw,
            'Z' => GameResult::Win,
            _ => bail!("Not a valid game result: {res:?}"),
        };
        Ok(res)
    };
    input
        .into_iter()
        .map(|(opp, res)| {
            parse_response(res)
                .map(|res| (opp, res))
                .map(|(o, r)| r as Int + o.response_for_result(r) as Int)
        })
        .sum::<Result<Int>>()
        .map(|x| x.to_string())
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
        indoc!(
            "
            A Y
            B X
            C Z
            "
        ),
        "15",
        "12",
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
