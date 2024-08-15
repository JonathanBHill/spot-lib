use color_eyre::eyre::Result;
use ratatui::{prelude::*, widgets::*};

use spot_lib_general::playlists::releaseradar::Release;

#[derive(Clone, Debug)]
pub struct ReleaseRadarTUI {
    // config: Config,
    #[allow(unused)]
    rr: Release
}
impl ReleaseRadarTUI {
    pub fn new(release: Release) -> Self {
        // let rr = ReleaseRadar::new();
        Self {
            rr: release
        }
    }
    fn widget_title(&self) -> Paragraph {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));
        
        Paragraph::new(Text::styled(
            "Release Radar",
            Style::default().fg(Color::LightGreen),
        )).alignment(Alignment::Center)
            .block(title_block)
    }
    #[allow(unused)]
    fn widget_last_updated(&self) -> Paragraph {
        let body_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .style(Style::default().fg(Color::Green));
        // let last_updated = self.rr.get_last_updated().await;
        let last_updated = "Test".to_string();
        Paragraph::new(last_updated)
            .block(body_block)
    }
    #[allow(unused)]
    fn last_updated_block(&self) -> Paragraph {
        let body_block = Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .style(Style::default().fg(Color::Green));
        Paragraph::new("\
        Last Updated: 2021-09-01\
        ")
            .block(body_block)
    }
    pub fn render(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        let full_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),  // Title
                    Constraint::Fill(1),  // Body
                ].as_ref()
            )
            .split(area);
        let body_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),  // Title
                    Constraint::Fill(1),  // Body
                ].as_ref()
            )
            .split(full_area[1]);
        #[allow(unused)]
        let rr_area = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(3),  // Title
                    Constraint::Fill(1),  // Body
                ].as_ref()
            )
            .split(body_area[1]);
        // let last_updated = self.widget_last_updated().await;
        frame.render_widget(self.widget_title(), full_area[0]);
        // frame.render_widget(self.widget_last_updated(), body_area[0]);
        Ok(())
    }
}
