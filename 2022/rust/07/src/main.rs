use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use aoc_parse::{parser, prelude::*};

static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../inputs/07"));

type Int = i64;
type Input = FileSystem;

#[derive(Debug, Clone, PartialEq, Eq)]
enum TerminalText {
    CdRoot,
    CdParent,
    Ls,
    CdDir { dir: String },
    File(File),
    Dir { name: String },
}

#[derive(Debug, Clone)]
struct FileSystem {
    root: FileTree,
}

#[derive(Debug, Clone, Default)]
struct FileTree {
    name: String,
    dirs: Vec<FileTree>,
    files: Vec<File>,
    size: Int,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct File {
    name: String,
    size: Int,
}

fn parse(input: &'static str) -> Result<Input> {
    let cmd_parser = parser!({
        "cd /" => TerminalText::CdRoot,
        "cd .." => TerminalText::CdParent,
        "ls" => TerminalText::Ls,
        "cd " dir:string(alnum+) => TerminalText::CdDir { dir }
    });
    let text_parser = parser!({
        "dir " name:string(alnum+) => TerminalText::Dir { name },
        size:i64 " " name:string(alnum+ ('.' alnum*)*) => TerminalText::File(File { name, size }),
        "$ " c:cmd_parser => c,
    });
    let p = parser!(lines(text_parser));
    let instructions = p.parse(input)?;

    let mut file_tree = FileSystem::default();
    let mut current_path = PathBuf::from("/");
    for instr in instructions {
        match instr {
            TerminalText::CdRoot => current_path.push("/"),
            TerminalText::CdParent => {
                current_path.pop();
            }
            TerminalText::Ls => (),
            TerminalText::CdDir { dir } => current_path.push(dir),
            TerminalText::File(file) => {
                let file_pointer = file_tree.get_pointer(&current_path)?;
                file_pointer.files.push(file);
            }
            TerminalText::Dir { name } => {
                let file_pointer = file_tree.get_pointer(&current_path)?;
                file_pointer.dirs.push(FileTree {
                    name,
                    ..Default::default()
                })
            }
        }
    }
    file_tree.fix_sizes();
    Ok(file_tree)
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            root: FileTree {
                name: "/".into(),
                dirs: vec![],
                files: vec![],
                size: 0,
            },
        }
    }
}

impl FileSystem {
    const MAX_TOTAL: Int = 70_000_000;
    const MIN_NEEDED: Int = 30_000_000;

    fn get_pointer(&mut self, path: &Path) -> Result<&mut FileTree> {
        let mut file_pointer = &mut self.root;
        for component in path.components() {
            match component {
                std::path::Component::RootDir => (), // RootDir will only come the first time and we are already pointing at root
                std::path::Component::Normal(name) => {
                    let child_pointer = file_pointer
                        .dirs
                        .iter_mut()
                        .find(|d| AsRef::<OsStr>::as_ref(&d.name) == name);
                    if let Some(child_pointer) = child_pointer {
                        file_pointer = child_pointer;
                    } else {
                        bail!("Directory {name:?} does not exist at current point {path:?}");
                    }
                }
                _ => bail!(
                    "We are on a very limited unix system. Got unexpected component: {component:?}"
                ),
            }
        }
        Ok(file_pointer)
    }

    fn fix_sizes(&mut self) {
        self.root.fix_sizes();
    }
}

impl FileTree {
    fn fix_sizes(&mut self) -> Int {
        if self.size != 0 {
            return self.size;
        }
        self.size = self.files.iter().map(|f| f.size).sum();
        self.size += self.dirs.iter_mut().map(Self::fix_sizes).sum::<Int>();
        self.size
    }
}

fn part1(file_tree: Input) -> Result<String> {
    fn count_total_100000(dir_block: &FileTree) -> Int {
        let mut count = dir_block.dirs.iter().map(count_total_100000).sum();
        if dir_block.size < 100000 {
            count += dir_block.size;
        }
        count
    }
    Ok(count_total_100000(&file_tree.root).to_string())
}

fn part2(file_tree: Input) -> Result<String> {
    let min_needed = FileSystem::MIN_NEEDED - (FileSystem::MAX_TOTAL - file_tree.root.size);
    fn search_min_size(dir_block: &FileTree, min_needed: Int) -> Option<&FileTree> {
        let block = dir_block
            .dirs
            .iter()
            .flat_map(|d| search_min_size(d, min_needed))
            .min_by_key(|d: &&FileTree| d.size);
        block.or_else(|| (dir_block.size >= min_needed).then_some(dir_block))
    }
    if min_needed <= 0 {
        Ok(0.to_string())
    } else {
        let Some(deleted_size) = search_min_size(&file_tree.root, min_needed).map(|d| d.size) else {
            bail!("Deleting the root directory must be enough to download the update");
        };
        Ok(deleted_size.to_string())
    }
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
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "},
        "95437",
        "24933642",
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
