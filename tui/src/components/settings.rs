use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

use crate::config::Config;
use crate::components::popups;
use crate::infrastructure::enums::SettingType;
use crate::infrastructure::widgets::Slider;

#[derive(Default, Clone, Debug)]
pub struct Settings {
    #[allow(unused)]
    config: Config,
    pub selected_setting: usize,
    selected_group: usize,
    groups: Vec<SettingsGroup>,
    pub input_active: bool,
    pub popup_active: bool,
    pub active_slider: Slider,
}

impl Settings {
    pub fn new() -> Self {
        let mut settings = Self::default();
        settings.selected_setting = 0;
        settings.selected_group = 0;
        settings.popup_active = false;
        settings.groups = vec![
            SettingsGroup::new(
                "General".to_string(),
                vec![
                    "Theme: Dark".to_string(),
                    "Font Size: 12".to_string(),
                ],
                vec![
                    SettingType::Category,
                    SettingType::Slider
                ],
            ),
            SettingsGroup::new(
                "Playback".to_string(),
                vec![
                    "Crossfade: Off".to_string(),
                    "Autoplay: On".to_string(),
                ],
                vec![
                    SettingType::Slider,
                    SettingType::OnOff
                ],
            ),
            SettingsGroup::new(
                "Playlists".to_string(),
                vec![
                    "Show Unplayable: Off".to_string(),
                    "Show Collaborative: On".to_string(),
                ],
                vec![
                    SettingType::OnOff,
                    SettingType::OnOff
                ],
            ),
            SettingsGroup::new(
                "Release Radar".to_string(),
                vec![
                    "Show New Releases: On".to_string(),
                    "Show New Singles: On".to_string(),
                ],
                vec![
                    SettingType::OnOff,
                    SettingType::OnOff
                ],
            ),
        ];
        settings
    }
    pub fn widget_title(&self) -> Paragraph {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Green));
        
        Paragraph::new(Text::styled(
            "Settings",
            Style::default().fg(Color::LightGreen),
        )).alignment(Alignment::Center)
            .block(title_block)
    }
    pub fn next_group(&mut self) {
        if self.selected_group < self.groups.len() - 1 {
            self.selected_group += 1;
        } else {
            self.selected_group = 0;
        }
    }
    pub fn previous_group(&mut self) {
        if self.selected_group > 0 {
            self.selected_group -= 1 % self.groups.len();
        } else {
            self.selected_group = self.groups.len() - 1;
        }
    }
    pub fn next_item(&mut self) {
        let group = &mut self.groups[self.selected_group];
            group.selected = (group.selected + 1) % group.settings.len();
    }
    pub fn previous_item(&mut self) {
        let group = &mut self.groups[self.selected_group];
        if group.selected > 0 {
            group.selected -= 1 % group.settings.len();
            // group.selected = (group.selected - 1) % group.settings.len();
        } else {
            group.selected = group.settings.len() - 1;
        }
    }
    fn toggle_on_off(&mut self) {
        let group = &mut self.groups[self.selected_group];
        let setting = &mut group.settings[group.selected];
        if setting.contains("On") {
            *setting = setting.replace("On", "Off");
        } else {
             *setting = setting.replace("Off", "On");
        }
    }
    fn toggle_slider(&mut self) {
        let group = &mut self.groups[self.selected_group];
        let setting = &mut group.settings[group.selected];
        if self.popup_active {
            if self.active_slider.title.contains("Crossfade") {
                if self.active_slider.current == 0 {
                    *setting = "Crossfade: Off".to_string();
                } else {
                    *setting = format!("Crossfade: {}s", self.active_slider.current);
                }
            } else if self.active_slider.title.contains("Font Size") {
                *setting = format!("Font Size: {}", self.active_slider.current);
            }
        }
        self.popup_active = !self.popup_active;
    }
    pub fn edit_setting(&mut self) {
        let group = &mut self.groups.clone()[self.selected_group];
        match group.settings_type[group.selected] {
            SettingType::OnOff => self.toggle_on_off(),
            SettingType::Slider => self.toggle_slider(),
            SettingType::Int => self.edit_value(),
            _ => (),
        };
    }
    fn edit_value(&mut self) {
    }
    #[allow(unused)]
    pub fn toggle_input_popup(&mut self) {
        self.popup_active = !self.popup_active;
    }
    fn generate_slider(&mut self, width: u16) -> Paragraph {
        let setting = &self.groups[self.selected_group].settings[self.groups[self.selected_group].selected];
        let title;
        if setting.contains("Crossfade") {
            title = "Edit Crossfade Duration";
            self.active_slider = Slider::new(
                0, 12, 0, self.active_slider.current,
                width,
                title,
                "Crossfade is off",
                Style::default().fg(Color::Green)
            );
        } else if setting.contains("Font Size") {
            title = "Edit Font Size";
            let val_parse = setting.split(": ").collect::<Vec<&str>>()[1].parse::<u16>().unwrap();
            self.active_slider = Slider::new(
                8, 24, val_parse, self.active_slider.current,
                width,
                title,
                "Font size: 12",
                Style::default().fg(Color::Green)
            );
        } else {
            title = "Edit Value";
        }
        let pp = self.active_slider.draw_slider_line();
        let popup = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Rgb(30,30,30)).fg(Color::LightGreen));
        let popup_text = Paragraph::new(pp.clone())
            .style(Style::default().fg(Color::LightGreen)).block(popup)
            .centered();
        popup_text
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
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
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(25),  // General
                    Constraint::Percentage(25),  // Playback
                    Constraint::Percentage(25),  // Playlists
                    Constraint::Percentage(25),  // Release Radar
                ].as_ref()
            )
            .split(full_area[1]);
        for (i, group) in self.groups.iter_mut().enumerate() {
            let title = if i == self.selected_group {
                let title_string = format!("{}{}", "▶ ".green(), &group.title);
                let title_string = format!("{}{}", title_string, " ◀".green());
                Span::raw(title_string).style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
            } else {
                Span::raw(group.title.clone()).style(Style::default().fg(Color::Green))
            };
            let body_block = Block::default()
                .title(title).title_alignment(Alignment::Center)
                .title_style(Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
                )
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Green));
            let items = group.settings.iter()
                .map(|i| ListItem::new(Span::raw(i)))
                .collect::<Vec<_>>();
            let mut state = ListState::default();
            state.select(Some(group.selected));
            
            let list = if i == self.selected_group {
                List::new(items.clone())
                    .block(body_block)
                    .highlight_style(
                        Style::default().add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol(">")
                    .style(Style::default().fg(Color::Green))
            } else {
                List::new(items.clone())
                    .block(body_block)
                    .highlight_style(
                        Style::default().add_modifier(Modifier::BOLD)
                    )
                    .highlight_symbol(">")
                    .style(Style::default().fg(Color::Gray))
            };
            let mut state = ListState::default();
            state.select(Some(group.selected));
            
            
            if self.input_active && i == self.selected_group {
                let input = Paragraph::new(Span::raw("Input: "))
                    .block(Block::default().borders(Borders::ALL).style(Style::default().fg(Color::Green)));
                frame.render_widget(input, body_area[i]);
            }
            frame.render_stateful_widget(list, body_area[i], &mut state);
        }
        if self.popup_active {
            let popup_area = popups::centered_rect(60, 20, frame.size());
            // let wid = self.generate_slider(popup_area.width);
            let group = &self.groups[self.selected_group];
            
            let widget = match group.settings_type[group.selected] {
                SettingType::Slider => self.generate_slider(popup_area.width),
                // SettingType::Int => Paragraph::new("int"),
                _ => Paragraph::new("default").block(Block::default().borders(Borders::ALL)),
            };
            // let pp = self.active_slider.draw_slider_line();
            // let popup = Block::default()
            //     .title(self.active_slider.title.as_str())
            //     .borders(Borders::ALL)
            //     .style(Style::default().bg(Color::Rgb(30,30,30)).fg(Color::LightGreen));
            // let popup_text = Paragraph::new(pp)
            //     .style(Style::default().fg(Color::LightGreen)).block(popup)
            //     .centered();
            frame.render_widget(Clear, popup_area);
            frame.render_widget(widget, popup_area);
            
            // frame.render_widget(popup_text, popup_area);
        }
        frame.render_widget(self.widget_title(), full_area[0]);
        Ok(())
    }
}

#[derive(Default, Clone, Debug)]
struct SettingsGroup {
    title: String,
    settings: Vec<String>,
    settings_type: Vec<SettingType>,
    selected: usize,
}
impl SettingsGroup {
    pub fn new(title: String, settings: Vec<String>, settings_type: Vec<SettingType>) -> Self {
        Self {
            title,
            settings,
            settings_type,
            selected: 0,
        }
    }
    #[allow(unused)]
    pub fn widget(&self) -> Paragraph {
        let body_block = Block::default()
            .title(Span::raw(&self.title)).bold().title_alignment(Alignment::Center)
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .style(Style::default().fg(Color::Green));
        let header_line = Line::from(
            Span::styled(
                &self.title,
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ),
        ).alignment(Alignment::Center);
        let mut text = Text::from(vec![header_line]);
        for setting in &self.settings {
            text.push_line(Line::from(Span::raw(setting)));
        }
        Paragraph::new(text).block(body_block)
    }
}
