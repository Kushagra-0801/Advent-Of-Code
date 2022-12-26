use anyhow::Result;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/01"));

type Int = i64;
type Input = Vec<Vec<Int>>;

fn parse(input: &'static str) -> Result<Input> {
    let mut splits = input.lines();
    let mut food_items = vec![];
    loop {
        let group = splits
            .by_ref()
            .take_while(|s| !s.is_empty())
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>();
        let group = group?;
        if group.is_empty() {
            break;
        }
        food_items.push(group)
    }
    Ok(food_items)
}

fn top_three_most_calories(input: &[Vec<Int>]) -> Result<[Int; 3]> {
    Ok(input.iter().map(|food| food.iter().sum::<Int>()).fold(
        [0, 0, 0],
        |mut top_three, cur_cal| {
            if top_three[2] > cur_cal {
            } else if top_three[1] > cur_cal {
                top_three[2] = cur_cal;
            } else if top_three[0] > cur_cal {
                top_three[2] = top_three[1];
                top_three[1] = cur_cal;
            } else {
                top_three[2] = top_three[1];
                top_three[1] = top_three[0];
                top_three[0] = cur_cal;
            }
            top_three
        },
    ))
}

fn part1(input: Input) -> Result<String> {
    Ok(top_three_most_calories(&input)?[0].to_string())
}

fn part2(input: Input) -> Result<String> {
    Ok(top_three_most_calories(&input)?
        .into_iter()
        .sum::<Int>()
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
        indoc!(
            "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            "
        ),
        "24000",
        "45000",
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
