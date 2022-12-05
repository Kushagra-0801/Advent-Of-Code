use std::{collections::HashSet, error::Error};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/03"));

type AocResult<T> = Result<T, Box<dyn Error>>;
type Int = u64;

fn get_priority(item: char) -> AocResult<Int> {
    match item {
        'a'..='z' => Ok((item as u8 - b'a' + 1).into()),
        'A'..='Z' => Ok((item as u8 - b'A' + 27).into()),
        _ => Err(format!("Invalid rucksack item: {item:?}").into()),
    }
}

fn part1(input: &str) -> AocResult<Int> {
    let mut total_priority = 0;
    let mut scratch = HashSet::with_capacity(52 / 2 + 1); // One partition will have half the items plus one repeat
    for rucksack in input.trim().lines() {
        if rucksack.is_empty() || rucksack.len() % 2 != 0 || !rucksack.is_ascii() {
            return Err(format!("Invalid rucksack configuration: {rucksack:?}").into());
        }
        let mid_pt = rucksack.len() / 2;
        let (compartment1, compartment2) = rucksack.split_at(mid_pt);
        scratch.clear();
        compartment1.chars().for_each(|c| {
            scratch.insert(c);
        });
        let repeat_item = compartment2
            .chars()
            .find(|c| scratch.contains(c))
            .ok_or_else(|| format!("No repeat item in rucksack: {rucksack:?}"))?;
        total_priority += get_priority(repeat_item)?;
    }
    Ok(total_priority)
}

fn part2(input: &str) -> AocResult<Int> {
    let mut total_priority = 0;
    let mut scratch1 = HashSet::with_capacity(52 / 3 + 1); // One partition will have a third of the items plus one badge
    let mut scratch2 = HashSet::with_capacity(52 / 3 + 1); // One partition will have a third of the items plus one badge
    let mut scratch3: HashSet<char> = HashSet::with_capacity(52 / 3 + 1); // One partition will have a third of the items plus one badge

    let mut rucksacks = input.trim().lines();
    while let Some(ruck1) = rucksacks.next() {
        let Some(ruck2) = rucksacks
            .next() else {
                return Err("Insufficient rucksacks in the group".to_string().into());
            };
        let Some(ruck3) = rucksacks
            .next() else {
                return Err("Insufficient rucksacks in the group".to_string().into());
            };
        scratch1.clear();
        scratch2.clear();
        scratch3.clear();
        ruck1.chars().for_each(|c| {
            scratch1.insert(c);
        });
        ruck2.chars().for_each(|c| {
            scratch2.insert(c);
        });
        scratch3.extend(scratch1.intersection(&scratch2));
        let group_badge = ruck3
            .chars()
            .find(|c| scratch3.contains(c))
            .ok_or_else(|| format!("No badge found in group: {ruck1:?} {ruck2:?} {ruck3:?}"))?;
        total_priority += get_priority(group_badge)?;
    }
    Ok(total_priority)
}

fn main() -> AocResult<()> {
    let part1 = part1(INPUT)?;
    let part2 = part2(INPUT)?;
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
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "},
        "157",
        "70",
    )];

    #[test]
    fn part1_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            assert_eq!(part1(case.input).unwrap().to_string(), case.output1);
        }
    }

    #[test]
    fn part2_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            assert_eq!(part2(case.input).unwrap().to_string(), case.output2);
        }
    }
}
