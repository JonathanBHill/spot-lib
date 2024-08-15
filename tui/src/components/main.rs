use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Stylize;

use spot_lib_general::playlists::releaseradar::Release;

use crate::components::about::About;
use crate::components::releaseradar::ReleaseRadarTUI;
use crate::components::settings::Settings;
use crate::infrastructure::enums::CurrentScreen;

#[derive(Clone, Debug)]
pub struct MainTUI {
    // config: Config,
    pub tab_labels: Vec<Line<'static>>,
    pub selected_tab: usize,
    #[allow(dead_code)]
    pub tabs: usize,
    pub current_screen: CurrentScreen,
    pub(crate) about: About,
    #[allow(dead_code)]
    pub layout: Layout,
    pub(crate) settings: Settings,
    pub(crate) release_radar: ReleaseRadarTUI,
    pub popup_active: bool,
}

impl MainTUI {
    pub fn new(layout: Layout, release_radar: Release) -> Self {
        let labels = vec![
            Line::from(vec![
                Span::styled("A", Style::default().add_modifier(Modifier::UNDERLINED).bold()),
                Span::raw("bout"),
            ]),
            Line::from(vec![
                Span::styled("C", Style::default().add_modifier(Modifier::UNDERLINED).bold()),
                Span::raw("urrently Playing"),
            ]),
            Line::from(vec![
                Span::styled("P", Style::default().add_modifier(Modifier::UNDERLINED).bold()),
                Span::raw("laylists"),
            ]),
            Line::from(vec![
                Span::styled("R", Style::default().add_modifier(Modifier::UNDERLINED).bold()),
                Span::raw("elease Radar"),
            ]),
            Line::from(vec![
                Span::styled("S", Style::default().add_modifier(Modifier::UNDERLINED).bold()),
                Span::raw("ettings"),
            ]),
        ];
        Self {
            tab_labels: labels.clone(),
            selected_tab: 0,
            tabs: labels.len(),
            current_screen: CurrentScreen::About,
            about: About::new(),
            layout,
            settings: Settings::new(),
            release_radar: ReleaseRadarTUI::new(release_radar),
            popup_active: false,
        }
        // home.tabs = home.tab_labels.len();
        // home.current_screen = CurrentScreen::About;
        // home.about = About::new();
        // home.settings = Settings::new();
        // home.release_radar = ReleaseRadarTUI::new();
        // home.layout = layout;
        // home.popup_active = false;
        // home
    }
    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.tab_labels.len();
        self.current_screen = match self.selected_tab {
            0 => CurrentScreen::About,
            1 => CurrentScreen::CurrentlyPlaying,
            2 => CurrentScreen::Playlists,
            3 => CurrentScreen::ReleaseRadar,
            4 => CurrentScreen::Settings,
            _ => CurrentScreen::About,
        };
    }
    pub fn set_tab(&mut self, tab: usize) {
        self.selected_tab = tab;
        self.current_screen = match self.selected_tab {
            0 => CurrentScreen::About,
            1 => CurrentScreen::CurrentlyPlaying,
            2 => CurrentScreen::Playlists,
            3 => CurrentScreen::ReleaseRadar,
            4 => CurrentScreen::Settings,
            _ => CurrentScreen::About,
        };
    }
    
    pub fn previous_tab(&mut self) {
        if self.selected_tab > 0 {
            self.selected_tab -= 1;
        } else {
            self.selected_tab = self.tab_labels.len() - 1;
        }
    }
    #[allow(dead_code)]
    fn widget_title(&self) -> Paragraph {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        
        let title = Paragraph::new(Text::styled(
            "The Spotify TUI",
            Style::default().fg(Color::LightGreen),
        )).alignment(Alignment::Center)
            .block(title_block);
        title
    }
    #[allow(dead_code)]
    pub fn widget_navigation_tabs(&self) -> Tabs {
        Tabs::new(self.tab_labels.clone())
            .block(Block::bordered().title("Navigation".light_green()).title_alignment(Alignment::Center).borders(Borders::ALL))
            .style(Style::default().fg(Color::Yellow))
            .highlight_style(Style::default().fg(Color::LightGreen).bg(Color::Black))
            .select(self.selected_tab)
            .divider(" | ")
            .padding("->", "<-")
    }
    #[allow(dead_code)]
    fn widget_nav_tab_as_block<'a>(&'a self, label: &'a str, selected: bool) -> Block {
        if selected {
            Block::default()
                .title(label)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Black).bg(Color::Yellow))
        } else {
            Block::default()
                .title(label)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
        }
    }
    
    fn nav_legend(&self) -> Paragraph {
        let legend_block = Block::default()
            .borders(Borders::TOP)
            .style(Style::default().fg(Color::Yellow));
        let line = Line::from(vec![
            "Press: ".yellow().bold(),
            "q".red(),
            " to quit the program".yellow(),
            " | ".yellow(),
            "->".red(),
            " or ".yellow(),
            "Tab".red(),
            " to navigate to the next tab".yellow(),
            " | ".yellow(),
            "<-".red(),
            " to navigate to the previous tab".yellow(),
        ]).alignment(Alignment::Center);
        let text = Text::from(line);
        let paragraph = Paragraph::new(text)
            .block(legend_block)
            .wrap(Wrap { trim: false });
        paragraph
    }
    
    #[allow(dead_code)]
    pub fn toggle_input_popup(&mut self) {
        self.popup_active = !self.popup_active;
    }
    pub(crate) fn render<'a>(&mut self, frame: &mut Frame<'a>) -> Result<()> {
        let full_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Navigation tabs
                Constraint::Fill(1),  // Main area
                Constraint::Length(3),  // Navigation legend
            ])
            .split(frame.size());
        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(full_area[1]);
        let tabs = self.widget_navigation_tabs();
        
        frame.render_widget(tabs, full_area[0]);
        frame.render_widget(self.nav_legend(), full_area[2]);
        match self.selected_tab {
            0 => {
                self.about.render(frame, full_area[1]).expect("Could not render about tab");
                // frame.render_widget(self.about.widget_title(), main_area[0]);
                // frame.render_widget(self.about.widget_body(), main_area[1]);
            }
            1 => {
                frame.render_widget(Paragraph::new("This is the currently playing screen"), main_area[0]);
            }
            2 => {
                frame.render_widget(Paragraph::new("This is the playlist screen"), main_area[0]);
            }
            3 => {
                self.release_radar.render(frame, full_area[1]).expect("Could not render release radar tab");
                // frame.render_widget(self.release_radar.widget_title(), main_area[0]);
            }
            4 => {
                self.settings.render(frame, full_area[1]).expect("Could not render settings tab");
                // frame.render_widget(Paragraph::new("This is the settings screen"), main_area[0]);
            }
            _ => {
                frame.render_widget(Paragraph::new("This is the default screen"), main_area[1]);
            }
        }
        // if self.popup_active {
        //     let popup_area = popups::centered_rect(60, 20, frame.size());
        //     let popup = Block::default()
        //         .title("Quit Menu")
        //         .borders(Borders::ALL)
        //         .style(Style::default().bg(Color::White).fg(Color::Black));
        //     let popup_text = Paragraph::new("This is a popup!")
        //         .style(Style::default().fg(Color::Black)).block(popup);
        //     frame.render_widget(popup_text, popup_area);
        //
        //     // frame.render_widget(popup_text, popup_area);
        // }
        Ok(())
    }
}
