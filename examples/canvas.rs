use std::io;

use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, MouseButton, MouseEventKind},
    execute, terminal,
};

use tui_canvas::{Cell, Grid};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        event::EnableMouseCapture
    )?;

    terminal::enable_raw_mode()?;

    let mut grid = Grid::new_full_screen()?;

    loop {
        grid.draw()?;
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
            Event::Mouse(event) => match event.kind {
                MouseEventKind::Down(mouse_button)
                | MouseEventKind::Drag(mouse_button) => {
                    let x = event.column / 2;
                    let y = event.row;

                    if mouse_button == MouseButton::Left {
                        let _ = grid.set_cell(x.into(), y.into(), Some(Cell::default()));
                    } else {
                        let _ = grid.set_cell(x.into(), y.into(), None);
                    }
                }
                _ => (),
            },
            _ => (),
        }
    }

    terminal::disable_raw_mode()?;

    execute!(
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
        event::DisableMouseCapture
    )
}
