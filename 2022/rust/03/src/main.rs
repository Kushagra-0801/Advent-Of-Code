use std::collections::HashSet;

use anyhow::{bail, Result};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/03"));

type Int = i64;

fn get_priority(item: char) -> Result<Int> {
    let priority = match item {
        'a'..='z' => item as u8 - b'a' + 1,
        'A'..='Z' => item as u8 - b'A' + 27,
        _ => bail!("Invalid rucksack item: {item:?}"),
    };
    Ok(priority.into())
}

fn part1(input: &str) -> Result<String> {
    let mut total_priority = 0;
    let mut scratch = HashSet::with_capacity(52 / 2 + 1); // One partition will have half the items plus one repeat
    for rucksack in input.trim().lines() {
        if rucksack.is_empty() || rucksack.len() % 2 != 0 || !rucksack.is_ascii() {
            bail!("Invalid rucksack configuration: {rucksack:?}");
        }
        let mid_pt = rucksack.len() / 2;
        let (compartment1, compartment2) = rucksack.split_at(mid_pt);
        scratch.clear();
        compartment1.chars().for_each(|c| {
            scratch.insert(c);
        });
        let Some(repeat_item) = compartment2.chars().find(|c| scratch.contains(c)) else {
            bail!("No repeat item in rucksack: {rucksack:?}");
        };
        total_priority += get_priority(repeat_item)?;
    }
    Ok(total_priority.to_string())
}

fn part2(input: &str) -> Result<String> {
    let mut total_priority = 0;
    let mut scratch1 = HashSet::with_capacity(52 / 3 + 1); // One partition will have a third of the items plus one badge
    let mut scratch2 = HashSet::with_capacity(52 / 3 + 1); // One partition will have a third of the items plus one badge
    let mut scratch3 = HashSet::with_capacity(52 / 3 + 1); // The intersection of the above two can only have a max of one third items

    let mut rucksacks = input.trim().lines();
    while let Some(ruck1) = rucksacks.next() {
        let Some(ruck2) = rucksacks.next() else { bail!("Insufficient rucksacks in the group") };
        let Some(ruck3) = rucksacks.next() else { bail!("Insufficient rucksacks in the group") };
        scratch1.clear();
        scratch2.clear();
        scratch3.clear();
        scratch1.extend(ruck1.chars());
        scratch2.extend(ruck2.chars());
        scratch3.extend(scratch1.intersection(&scratch2).copied());
        let Some(group_badge) = ruck3.chars().find(|c| scratch3.contains(c)) else {
            bail!("No badge found in group: {ruck1:?} {ruck2:?} {ruck3:?}");
        };
        total_priority += get_priority(group_badge)?;
    }
    Ok(total_priority.to_string())
}

fn main() -> Result<()> {
    let input = INPUT;
    let part1 = part1(input)?;
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
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "},
        "157",
        "70",
    ]];

    #[test]
    fn part1_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            let input = case.input;
            let output = part1(input).unwrap();
            assert_eq!(output, case.out1, "Failed for {case:?}");
        }
    }

    #[test]
    fn part2_test() {
        for case in TEST_CASES {
            let case = TestCase::from(case);
            let input = case.input;
            let output = part2(input).unwrap();
            assert_eq!(output, case.out2, "Failed for {case:?}");
        }
    }
}
