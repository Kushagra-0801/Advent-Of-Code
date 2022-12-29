use anyhow::Result;
use aoc_parse::{parser, prelude::*};
use aoc_utils::grids::{Grid, Idx};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/08"));

type TreeCover = Grid<u8>;
type Input = TreeCover;

fn parse(input: &'static str) -> Result<Input> {
    let p = parser!(lines({h:digit => h as u8}+));
    let grid = p
        .parse(input)
        .map(|grid| grid.into_iter().flatten().collect())?;
    let columns = input
        .lines()
        .next()
        .map(str::len)
        .expect("The parser worked above so input cannot have been empty");
    let grid = Grid::try_from_vec(grid, columns)?;
    Ok(grid)
}

fn is_tree_visible(tree: Idx, grid: &TreeCover) -> bool {
    let tree_height = grid[tree];

    let col_indexer = |col| Idx { row: tree.row, col };
    let trees_in_row = [
        (0..tree.col).map(col_indexer),                // left
        (tree.col + 1..grid.columns).map(col_indexer), // right
    ];
    let row_indexer = |row| Idx { row, col: tree.col };
    let trees_in_col = [
        (0..tree.row).map(row_indexer),             // up
        (tree.row + 1..grid.rows).map(row_indexer), // down
    ];

    trees_in_row
        .into_iter()
        .any(|mut t| t.all(|x| grid[x] < tree_height))
        || trees_in_col
            .into_iter()
            .any(|mut t| t.all(|x| grid[x] < tree_height))
}

fn part1(tree_cover: Input) -> Result<String> {
    let visible_trees = (0..tree_cover.rows)
        .flat_map(|r| (0..tree_cover.columns).map(move |c| (r, c)))
        .map(|(row, col)| Idx { row, col })
        .map(|tree| is_tree_visible(tree, &tree_cover))
        .map(Into::<i64>::into)
        .sum::<i64>();
    Ok(visible_trees.to_string())
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
            30373
            25512
            65332
            33549
            35390
        "},
        "21",
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
