use std::io;

use crossterm::event::{read, Event, MouseButton, MouseEventKind};

use crate::Grid;

/// Gets the x, y coordinates of a mouse click in relation to the grid.
///
/// The conversion from actual x, y to the pixel grid is automatic
///
/// # Parameters
///
/// - `grid` The grid to get the click on
/// - `button` The mouse button to get the click of
///
/// # Returns
///
/// `None` if the click is outside the grid, otherwise a tuple with the format (x, y)
pub fn get_mouse_click(grid: &Grid, button: MouseButton) -> io::Result<Option<(usize, usize)>> {
    if let Event::Mouse(event) = read()? {
        if event.kind == MouseEventKind::Down(button) {
            // A cell is 2 columns wide
            let x = (event.column / 2) as usize;
            let y = event.row as usize;

            // Clicks outside of the grid don't count
            if x >= grid.width() && y >= grid.height() {
                return Ok(None);
            }

            return Ok(Some((x, y)));
        }
    }
    Ok(None)
}

/// Gets the x, y coordinates of a mouse click in relation to the grid.
///
/// The conversion from actual x, y to the pixel grid is automatic
///
/// # Parameters
///
/// - `grid` The grid to get the click on
/// - `button` The mouse button to get the click of
///
/// # Returns
///
/// `None` if the click is outside the grid, otherwise a tuple with the format (x, y)
pub fn get_mouse_click_or_drag(
    grid: &Grid,
    button: MouseButton,
) -> io::Result<Option<(usize, usize)>> {
    if let Event::Mouse(event) = read()? {
        if event.kind == MouseEventKind::Down(button) || event.kind == MouseEventKind::Drag(button)
        {
            // A cell is 2 columns wide
            let x = (event.column / 2) as usize;
            let y = event.row as usize;

            // Clicks outside of the grid don't count
            if x >= grid.width() && y >= grid.height() {
                return Ok(None);
            }

            return Ok(Some((x, y)));
        }
    }
    Ok(None)
}
