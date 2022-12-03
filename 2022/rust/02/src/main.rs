use std::error::Error;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/02"));

type AocResult<T> = Result<T, Box<dyn Error>>;

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
}

impl TryFrom<char> for RockPaperScissors {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let play = match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => return Err(format!("Not a rock paper scissors move: {value:?}")),
        };
        Ok(play)
    }
}

type Int = u64;
type Input = Vec<(RockPaperScissors, char)>;

fn parse_input(input: &str) -> AocResult<Input> {
    let mut moves = vec![];
    for line in input.lines() {
        let Some((opponent, response)) = line.split_once(' ') else { return Err(format!("Not a valid input line: {line:?}").into()) };
        let opponent = if opponent.len() == 1 {
            opponent.chars().next().unwrap()
        } else {
            return Err(format!("Invalid Opponent move: {opponent:?}").into());
        };
        let response = if response.len() == 1 {
            response.chars().next().unwrap()
        } else {
            return Err(format!("Invalid response move: {response:?}").into());
        };
        let opponent = opponent.try_into()?;
        moves.push((opponent, response));
    }
    Ok(moves)
}

fn part1(input: &Input) -> AocResult<Int> {
    fn parse_response(res: char) -> AocResult<RockPaperScissors> {
        let res = match res {
            'X' => RockPaperScissors::Rock,
            'Y' => RockPaperScissors::Paper,
            'Z' => RockPaperScissors::Scissors,
            _ => return Err(format!("Not a rock paper scissors move: {res:?}").into()),
        };
        Ok(res)
    }
    input
        .iter()
        .map(|&(opp, res)| {
            parse_response(res)
                .map(|res| (opp, res))
                .map(|(o, r)| r as Int + r.play_against(o) as Int)
        })
        .sum()
}

fn main() -> AocResult<()> {
    let input = parse_input(INPUT)?;
    let part1 = part1(&input)?;
    // let part2 = part2(&input)?;
    println!("Part 1: {part1}");
    // println!("Part 2: {part2}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    struct TestCase {
        input: &'static str,
        output1: &'static str,
        output2: &'static str,
    }

    impl From<(&'static str, &'static str, &'static str)> for TestCase {
        fn from((i, o1, o2): (&'static str, &'static str, &'static str)) -> Self {
            Self {
                input: i,
                output1: o1,
                output2: o2,
            }
        }
    }

    static TEST_CASES: [(&str, &str, &str); 1] = [(
        indoc! {"
            A Y
            B X
            C Z
        "},
        "15",
        "",
    )];

    #[test]
    fn part1_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            let input = parse_input(case.input).unwrap();
            assert_eq!(part1(&input).unwrap().to_string(), case.output1);
        }
    }

    // #[test]
    // fn part2_test() {
    //     for case in TEST_CASES {
    //         let case = TestCase::from(case);
    //         let input = parse_input(case.input).unwrap();
    //         assert_eq!(part2(&input).unwrap().to_string(), case.output2);
    //     }
    // }
}
