use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

use crate::config::Config;

#[derive(Default, Clone, Debug)]
pub struct About {
    #[allow(unused)]
    config: Config,
}
impl About {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn widget_title(&self) -> Paragraph {
        let title_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
            .style(Style::default().fg(Color::Green));
        
        Paragraph::new(Text::styled(
            "The Spotify TUI",
            Style::default().fg(Color::LightGreen),
        )).alignment(Alignment::Center)
            .block(title_block)
    }
    pub fn widget_body(&self) -> Paragraph {
        let body_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .style(Style::default().fg(Color::Green));
        Paragraph::new("\
        Welcome! This is a text-based user interface for viewing or managing different aspects of your Spotify account including:\n\
        - Currently playing tracks\n- Playlists\n- Release Radar\n\
        \n\
        You can return to the about screen by pressing 'a' or 1.\n\
        You can navigate to the currently playing screen by pressing 'c' or 2.\n\
        You can navigate to the playlist screen by pressing 'p' or 3.\n\
        You can navigate to the release radar screen by pressing 'r' or 4.\n\
        You can navigate to the TUI settings by pressing 's' or 5.\n\n\
        Thank you for trying out Spotui!\
        ")
            .block(body_block)
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let body_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),  // Title
                    Constraint::Fill(1),  // Body
                ].as_ref()
            )
            .split(area);
        frame.render_widget(self.widget_title(), body_area[0]);
        frame.render_widget(self.widget_body(), body_area[1]);
        Ok(())
    }
    }
