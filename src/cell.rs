use crossterm::style::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    color: Color,
    value: String,
}

impl Cell {
    /// Attempts to build a new [`Cell`]
    ///
    /// # Parameters
    ///
    /// - `color` The color of the cell, which is a [`Color`]
    /// - `value` The value the cell holds. This is what is printed to the screen
    ///
    /// # Returns
    ///
    /// `None` if the value is not a [`String`] of length 2
    pub fn build(color: Color, value: impl Into<String>) -> Option<Self> {
        let value = value.into();

        match value.len() == 2 {
            true => Some(Self { color, value }),
            false => None,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn value(&self) -> &str {
        self.value.as_ref()
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::build(Color::White, "  ").expect("Cell will always build with these inputs")
    }
}

#[cfg(test)]
mod tests {
    use crossterm::style::Color;

    use super::Cell;

    #[test]
    fn cell_build_works() {
        let cell = Cell::build(Color::White, "  ");

        assert!(cell.is_some())
    }

    #[test]
    fn cell_build_fails() {
        let cell = Cell::build(Color::White, " ");

        assert!(cell.is_none())
    }

    #[test]
    fn default_cell_works() {
        Cell::default();
    }
}
