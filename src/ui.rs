use color_eyre::owo_colors::colors::Red;
use color_eyre::owo_colors::{Color, OwoColorize, style};
use ratatui::Frame;
use ratatui::style::Styled;
use ratatui::widgets::{List, ListItem, Paragraph};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::app::{App, AppFocus};

trait UiElement {
    fn render(&self, f: &mut Frame, area: Rect);
}

enum TextFocus {
    none,
    hover,
    selected,
}

struct TextElement {
    pub text: String,
    pub focus: TextFocus,
}

impl UiElement for TextElement {
    fn render(&self, f: &mut Frame, area: Rect) {
        let text_style = Style::default().add_modifier(match self.focus {
            TextFocus::selected => Modifier::UNDERLINED,
            _ => Modifier::empty(),
        });
        let para = Paragraph::new(self.text.clone()).set_style(text_style);
        f.render_widget(para, area);
    }
}

struct BlockContainer {
    pub title: String,
    pub focused: bool,
    pub elements: Vec<Box<dyn UiElement>>,
}

impl UiElement for BlockContainer {
    fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(match self.focused {
                true => Style::default().blue(),
                false => Style::default().white(),
            });
        f.render_widget(block.clone(), area);

        let inner = block.inner(area);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1); self.elements.len()])
            .split(inner);

        for (i, el) in self.elements.iter().enumerate() {
            el.render(f, chunks[i]);
        }
    }
}

struct WindowLayout {
    playlists: Rect,
    tracks: Rect,
    now_playing: Rect,
    status: Rect,
}

fn create_splits(size: Rect) -> WindowLayout {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(size);
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(vertical_chunks[0]);

    WindowLayout {
        playlists: top_chunks[0],
        tracks: top_chunks[1],
        now_playing: top_chunks[2],
        status: vertical_chunks[1],
    }
}

pub fn render(f: &mut Frame, app: &App) -> () {
    let size = f.area();
    let panes = create_splits(size);

    let playlist_container = BlockContainer {
        title: "Playlists".into(),
        focused: app.focus == AppFocus::playlist,
        elements: app
            .playlists
            .iter()
            .map(|playlist| {
                Box::new(TextElement {
                    text: playlist.name.clone(),
                    focus: match app.selected_playlist == playlist.name {
                        true => TextFocus::selected,
                        false => TextFocus::none,
                    },
                }) as Box<dyn UiElement>
            })
            .collect(),
    };
    playlist_container.render(f, panes.playlists);

    let track_container = BlockContainer {
        title: "Tracks".into(),
        focused: app.focus == AppFocus::tracks,
        elements: app
            .playlists
            .iter()
            .map(|playlist| {
                Box::new(TextElement {
                    text: playlist.name.clone(),
                    focus: match app.selected_playlist == playlist.name {
                        true => TextFocus::selected,
                        false => TextFocus::none,
                    },
                }) as Box<dyn UiElement>
            })
            .collect(),
    };
    track_container.render(f, panes.tracks);

    let now_playing_container = BlockContainer {
        title: "Now Playing".into(),
        focused: app.focus == AppFocus::now_playing,
        elements: vec![],
    };
    now_playing_container.render(f, panes.now_playing);

    let status_container = BlockContainer {
        title: "Status".into(),
        focused: app.focus == AppFocus::status,
        elements: vec![],
    };
    status_container.render(f, panes.status);
}
