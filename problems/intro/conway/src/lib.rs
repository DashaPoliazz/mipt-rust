#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////
/// 1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
/// 2. Any live cell with two or three live neighbors lives on to the next generation.
/// 3. Any live cell with more than three live neighbors dies, as if by over-population.
/// 4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Grid {
            cols,
            rows,
            grid: vec![T::default(); rows * cols],
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        // TODO: your code goes here.
        Grid {
            cols,
            rows,
            grid: grid.to_vec(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        // TODO: your code goes here.
        return (self.rows, self.cols);
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        // TODO: your code goes here.
        let idx = self.adjacent_idx(row, col);
        return &self.grid[idx];
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        // TODO: your code goes here.
        let idx = self.adjacent_idx(row, col);
        self.grid[idx] = value;
    }

    pub fn is_in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.rows && col < self.cols
    }
    // ROW => y, COL => x
    pub fn neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        // TODO: your code goes here.
        let mut out = vec![];
        let directions: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            /* (0,0) */ (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dy, dx) in directions {
            let new_row = (row as isize).checked_sub(dy);
            let new_col = (col as isize).checked_sub(dx);

            if let (Some(new_row), Some(new_col)) = (new_row, new_col) {
                if new_row >= 0 && new_col >= 0 {
                    let new_row = new_row as usize;
                    let new_col = new_col as usize;

                    if self.is_in_bounds(new_row, new_col) {
                        out.push((new_row, new_col));
                    }
                }
            }
        }

        return out;
    }

    fn adjacent_idx(&self, row: usize, col: usize) -> usize {
        return (self.rows * row) + col;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn step(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}
