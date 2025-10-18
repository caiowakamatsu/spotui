use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io;

enum Focus {
    Increment,
    Decrement,
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut terminal = ratatui::init();

    let mut count: i32 = 0;
    let mut focus = Focus::Increment;

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(size);

            let counter = Paragraph::new(format!("Count: {}", count))
                .alignment(Alignment::Center)
                .block(Block::default().title("Counter").borders(Borders::ALL));
            f.render_widget(counter, chunks[0]);

            let button_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

            let inc_block = Block::default()
                .title("Increment")
                .borders(Borders::ALL)
                .border_style(match focus {
                    Focus::Increment => Style::default().fg(Color::Yellow),
                    _ => Style::default().fg(Color::Gray),
                });

            let dec_block = Block::default()
                .title("Decrement")
                .borders(Borders::ALL)
                .border_style(match focus {
                    Focus::Decrement => Style::default().fg(Color::Yellow),
                    _ => Style::default().fg(Color::Gray),
                });

            f.render_widget(inc_block, button_layout[0]);
            f.render_widget(dec_block, button_layout[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right | KeyCode::Left => {
                        focus = match focus {
                            Focus::Increment => Focus::Decrement,
                            Focus::Decrement => Focus::Increment,
                        };
                    }
                    KeyCode::Enter => match focus {
                        Focus::Increment => count += 1,
                        Focus::Decrement => count -= 1,
                    },
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    ratatui::restore();
    Ok(())
}
