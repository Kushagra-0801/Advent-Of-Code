use std::error::Error;

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/01"));

type Int = u64;
type AocResult<T> = Result<T, Box<dyn Error>>;

fn parse_input(input: &str) -> AocResult<Vec<Vec<Int>>> {
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

fn top_three_most_calories(input: &[Vec<Int>]) -> AocResult<[Int; 3]> {
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

fn part1(input: &[Vec<Int>]) -> AocResult<Int> {
    Ok(top_three_most_calories(input)?[0])
}

fn part2(input: &[Vec<Int>]) -> AocResult<Int> {
    Ok(top_three_most_calories(input)?.into_iter().sum())
}

fn main() -> AocResult<()> {
    let input = parse_input(INPUT)?;
    let part1 = part1(&input)?;
    let part2 = part2(&input)?;
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
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
        },
        "24000",
        "45000",
    )];

    #[test]
    fn part1_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            let input = parse_input(case.input).unwrap();
            assert_eq!(part1(&input).unwrap().to_string(), case.output1);
        }
    }

    #[test]
    fn part2_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            let input = parse_input(case.input).unwrap();
            assert_eq!(part2(&input).unwrap().to_string(), case.output2);
        }
    }
}
