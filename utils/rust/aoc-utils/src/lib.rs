pub mod grids {
    use std::ops::{Index, IndexMut};

    use anyhow::{ensure, Result};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Idx {
        pub row: usize,
        pub col: usize,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    pub struct Grid<T> {
        pub grid: Vec<T>,
        pub columns: usize,
        pub rows: usize,
    }

    impl<T> Grid<T> {
        pub fn with_capacity(rows: usize, cols: usize) -> Self {
            Self {
                grid: Vec::with_capacity(rows * cols),
                columns: 0,
                rows: 0,
            }
        }

        pub fn try_from_vec(grid: Vec<T>, columns: usize) -> Result<Self> {
            ensure!(
                grid.len() % columns == 0,
                "Provided vector can not be fit into a rectangular grid"
            );
            let rows = grid.len() / columns;
            Ok(Self {
                grid,
                columns,
                rows,
            })
        }
    }

    impl<T> Index<usize> for Grid<T> {
        type Output = [T];

        fn index(&self, index: usize) -> &Self::Output {
            let start = index * self.columns;
            let end = start + self.columns;
            &self.grid[start..end]
        }
    }

    impl<T> IndexMut<usize> for Grid<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            let start = index * self.columns;
            let end = start + self.columns;
            &mut self.grid[start..end]
        }
    }

    impl<T> Index<Idx> for Grid<T> {
        type Output = T;

        fn index(&self, index: Idx) -> &Self::Output {
            &self[index.row][index.col]
        }
    }

    impl<T> IndexMut<Idx> for Grid<T> {
        fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
            &mut self[index.row][index.col]
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TestCase {
    pub input: &'static str,
    pub out1: &'static str,
    pub out2: &'static str,
}

impl From<[&'static str; 3]> for TestCase {
    fn from(value: [&'static str; 3]) -> Self {
        Self::from(&value)
    }
}

impl From<&[&'static str; 3]> for TestCase {
    fn from(value: &[&'static str; 3]) -> Self {
        Self {
            input: value[0],
            out1: value[1],
            out2: value[2],
        }
    }
}
