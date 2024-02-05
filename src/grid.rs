use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetBackgroundColor},
};

use crate::cell::Cell;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub type GridResult = Result<(), &'static str>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    /// A 2D grid of cells that will be drawn to the terminal
    grid: Vec<Vec<Option<Cell>>>,
    /// The buffered changes to the grid
    changes: Vec<Point>,
    /// The width of the grid
    width: usize,
    /// The height of the grid
    height: usize,
}

impl Grid {
    /// Creates a new Grid
    ///
    /// # Panics
    ///
    /// Panics if the width or the height is less than 1
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let grid = vec![vec![None; width]; height];

        Grid {
            grid,
            changes: vec![],
            width,
            height,
        }
    }

    /// Attempts to create a grid the size of the user's terminal window
    pub fn new_full_screen() -> io::Result<Self> {
        let (width, height) = crossterm::terminal::size()?;

        let actual_width = (width - 1) as usize;
        let actual_height = (height - 1) as usize;

        Ok(Self::new(actual_width, actual_height))
    }

    /// Sets the given cell to true if the cell is in range
    ///
    /// # Parameters
    ///
    /// - `x` The column of the cell to toggle, with 0 being the leftmost cell
    /// - `y` The row of the cell to toggle, with 0 being the top of the screen
    /// - `cell` The [`Cell`] to set the cell to, or `None` to unset the cell
    ///
    /// # Returns
    ///
    /// An [`Err`] if the cell is out of range
    pub fn set_cell(&mut self, x: usize, y: usize, cell: Option<Cell>) -> GridResult {
        if x >= self.width || y >= self.height {
            return Err("Cell outside of range");
        }

        self.grid[y][x] = cell.clone();
        self.changes.push(Point::new(x, y));

        Ok(())
    }

    /// Gets the value of the cell at the given coordinates
    ///
    /// # Parameters
    ///
    /// - `x` The column of the cell to toggle, with 0 being the leftmost cell
    /// - `y` The row of the cell to toggle, with 0 being the top of the screen
    ///
    /// # Returns
    ///
    /// The `None` if the cell is out of range of the grid, otherwise the the cell
    /// wrapped in `Some()`
    pub fn get_cell(&mut self, x: usize, y: usize) -> Option<Cell> {
        self.grid.get(y)?.clone().get(x)?.clone()
    }

    /// Draws the grid to the terminal
    pub fn draw(&mut self) -> io::Result<()> {
        let mut stdout = stdout();

        if self.changes.is_empty() {
            return self.draw_all(&mut stdout);
        }

        self.draw_changes(&mut stdout)
    }

    /// Draws only the changes to the grid
    fn draw_changes(&mut self, stdout: &mut Stdout) -> io::Result<()> {
        for change in &self.changes {
            let x = change.x;
            let y = change.y;

            let x_u16 = x.try_into().expect("This should never fail");
            let y_u16 = y.try_into().expect("This should never fail");

            match &self.grid[y][x] {
                Some(cell) => draw_cell(stdout, x_u16, y_u16, cell)?,
                None => erase_cell(stdout, x_u16, y_u16)?,
            }
        }

        self.changes = vec![];

        Ok(())
    }

    /// Draws the whole grid to the terminal, which is used when there have been no changes made
    /// yet
    fn draw_all(&self, stdout: &mut Stdout) -> io::Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                let x_u16 = x.try_into().expect("This should never fail");
                let y_u16 = y.try_into().expect("This should never fail");

                match &self.grid[y][x] {
                    Some(cell) => draw_cell(stdout, x_u16, y_u16, cell)?,
                    None => erase_cell(stdout, x_u16, y_u16)?,
                }
            }
        }

        Ok(())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(80, 50)
    }
}

/// Draws a cell at the given coordinates
///
/// **NOTE** A cell is actually two chars wide, to make a square.
///          This is accounted for in the function.
///
/// # Parameters
///
/// - `x`    The column to draw the cell on, with 0 being the leftmost cell
/// - 'y'    The row to draw the cell on, with 0 being the top row
/// - `cell` The cell to draw
fn draw_cell(stdout: &mut Stdout, x: u16, y: u16, cell: &Cell) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(x * 2, y),
        SetBackgroundColor(cell.color()),
        Print(cell.value()),
        SetBackgroundColor(Color::Reset),
    )?;

    stdout.flush()
}

/// Erases a cell at the given coordinates
///
/// **NOTE** A cell is actually two chars wide, to make a square.
///          This is accounted for in the function.
///
/// # Parameters
///
/// - `x` The column of the cell to draw, with 0 being the leftmost cell
/// - `y` The row of the cell to draw, with 0 being the top of the screen
fn erase_cell(stdout: &mut Stdout, x: u16, y: u16) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(x * 2, y),
        SetBackgroundColor(Color::Reset),
        Print("  "),
    )?;

    stdout.flush()
}

#[cfg(test)]
mod tests {
    use crate::Cell;

    use super::Grid;

    #[test]
    fn grid_new_works() {
        let grid = Grid::new(3, 2);

        let expected = vec![vec![None, None, None], vec![None, None, None]];

        assert_eq!(
            grid,
            Grid {
                grid: expected,
                changes: vec![],
                width: 3,
                height: 2
            }
        );
    }

    #[test]
    fn grid_set_cell_works() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(1, 2, Some(Cell::default())).unwrap();

        let expected = vec![
            vec![None, None, None],
            vec![None, None, None],
            vec![None, Some(Cell::default()), None],
        ];

        assert_eq!(grid.grid, expected);
    }

    #[test]
    fn grid_get_cell_works() {
        let mut grid = Grid::new(3, 3);

        grid.set_cell(1, 2, Some(Cell::default())).unwrap();

        assert_eq!(
            grid.get_cell(1, 2).expect("Failed to unwrap get_cell"),
            Cell::default()
        )
    }

    #[test]
    fn grid_get_cell_out_of_bounds_is_none() {
        let mut grid = Grid::new(3, 3);

        assert!(grid.get_cell(5, 3).is_none());
    }
}
