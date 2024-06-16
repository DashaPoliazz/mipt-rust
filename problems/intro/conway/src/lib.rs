#![forbid(unsafe_code)]

use std::fmt;

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

impl<T: fmt::Debug + Clone + Default> fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:?} ", self.get(row, col))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
enum Existance {
    Live,
    Die,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        // TODO: your code goes here.
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        // TODO: your code goes here.
        &self.grid
    }

    pub fn step(&mut self) {
        /*
         1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
         2. Any live cell with two or three live neighbors lives on to the next generation.
         3. Any live cell with more than three live neighbors dies, as if by over-population.
         4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
        */
        let (rows, cols) = self.grid.size();
        let mut graveyard: Vec<(usize, usize)> = vec![];
        let mut heaven: Vec<(usize, usize)> = vec![];
        for row in 0..rows {
            for col in 0..cols {
                let alive_neighbours = self
                    .grid
                    .neighbours(row, col)
                    .iter()
                    .filter(|(row, col)| *self.grid.get(*row, *col) == Cell::Alive)
                    .count();

                match (self.grid.get(row, col), alive_neighbours) {
                    // Rule 1 and Rule 3 for live cells
                    (Cell::Alive, 0..=1) | (Cell::Alive, 4..=8) => graveyard.push((row, col)),
                    // Rule 2 for live cells
                    (Cell::Alive, 2) | (Cell::Alive, 3) => (),
                    // Rule 4 for dead cells
                    (Cell::Dead, 3) => heaven.push((row, col)),
                    _ => (),
                }
            }
        }
        // Killing cells
        graveyard
            .iter()
            .for_each(|(row, col)| self.grid.set(Cell::Dead, *row, *col));
        // Ressurecting cells
        heaven
            .iter()
            .for_each(|(row, col)| self.grid.set(Cell::Alive, *row, *col));
    }
}
