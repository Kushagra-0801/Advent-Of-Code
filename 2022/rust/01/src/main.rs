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

fn part1(input: &[Vec<Int>]) -> AocResult<Int> {
    input
        .iter()
        .map(|food| food.iter().sum::<Int>())
        .max()
        .ok_or_else(|| "No elves = no food items".into())
}

fn main() -> AocResult<()> {
    let input = parse_input(INPUT)?;
    let part1 = part1(&input)?;
    println!("Part 1: {part1}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    struct TestCase {
        input: &'static str,
        output: &'static str,
    }

    impl From<(&'static str, &'static str)> for TestCase {
        fn from((i, o): (&'static str, &'static str)) -> Self {
            Self {
                input: i,
                output: o,
            }
        }
    }

    static TEST_CASES_PART_1: [(&str, &str); 1] = [(
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
    )];

    #[test]
    fn part1_test() {
        for case in TEST_CASES_PART_1 {
            let case = TestCase::from(case);
            let input = parse_input(case.input).unwrap();
            assert_eq!(part1(&input).unwrap().to_string(), case.output);
        }
    }
}
