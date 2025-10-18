use std::io;

use crate::app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

mod app;
mod ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = ratatui::init();

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            ui::render(f, &app);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app.focus_left(),
                    KeyCode::Char('l') => app.focus_right(),
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    ratatui::restore();
    Ok(())
}
