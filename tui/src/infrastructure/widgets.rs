use ratatui::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Slider {
    pub min: u16,
    pub max: u16,
    pub current: u16,
    pub width: u16,
    pub title: String,
    pub status_string: String,
    #[allow(unused)]
    pub style: Style,
}

impl Slider {
    pub fn new(
        min: u16,
        max: u16,
        default: u16,
        mut current: u16,
        width: u16,
        title: &str,
        status_string: &str,
        style: Style,
    ) -> Self {
        // let mut current;
        if current < min {
            current = default;
            // panic!("Current value cannot be less than the minimum value");
        } else if current > max {
            current = default;
            // panic!("Current value cannot be greater than the maximum value");
        } else {
            current = current;
        }
        // if default == 6969 {
        //     current = min;
        // }
        Slider {
            min,
            max,
            current,
            width,
            title: title.to_string(),
            status_string: status_string.to_string(),
            style,
        }
    }
    pub fn set_current(&mut self, value: u16) {
        self.current = value;
    }
    pub fn increase(&mut self) {
        if self.current < self.max {
            let current = self.current + 1;
            self.set_current(current);
        }
    }
    pub fn decrease(&mut self) {
        if self.current > self.min {
            let current = self.current - 1;
            self.set_current(current);
        }
    }
    pub fn draw_slider_line(&mut self) -> Text {
        let range = self.max - self.min;
        let value = self.current - self.min;
        let width = self.width as f64;
        let fill = (value as f64 / range as f64 * (width - 7.0)).round() as u16;
        let mut line = String::new();
        let min_as_string = self.min.to_string();
        let max_as_string = self.max.to_string();
        for character in min_as_string.chars().into_iter() {
            line.push(character);
        }
        line.push(' ');
        
        for _ in 0..fill {
            line.push('█');
        }
        for _ in fill..(self.width - 7) {
            line.push('░');
        }
        line.push(' ');
        for character in max_as_string.chars().into_iter() {
            line.push(character);
        }
        let line = Line::from(line).alignment(Alignment::Center);
        
        let status_line = self.generate_status_line();
        
        Text::from(vec![line, status_line])
    }
    fn generate_status_line(&mut self) -> Line {
        let mut current_value = self.status_string.clone();
        if self.status_string.contains("Crossfade") {
            if self.current == 0 {
                current_value = "Crossfade is off".to_string();
                self.status_string = current_value.clone();
            } else if self.current == 1 {
                current_value = "Crossfade set to 1 second".to_string();
                // current_value = current_value.replace("seconds", "second");
                self.status_string = current_value.clone();
            } else if self.current > 1 {
                current_value = format!("Crossfade set to {} seconds", self.current);
                // current_value = current_value.replace("seconds", "second");
                self.status_string = current_value.clone();
            }
        } else if self.status_string.contains("Font size") {
            current_value = format!("Set font size to {}", self.current);
            self.status_string = current_value.clone();
        }
        // self.status_string = *current_value.clone().as_str();
        Line::from(current_value).bold().alignment(Alignment::Center)
    }
    // pub fn generate_slider_block(self, area: Rect) -> Paragraph {
    //     let block = Block::default()
    //         .title(self.title.as_str())
    //         .borders(Borders::ALL)
    //         .style(self.style);
    //     let slider_line = self.draw_slider_line();
    //     Paragraph::new(slider_line)
    //         .block(block)
    //         .wrap(Wrap { trim: false });
    //
    //     // slider_line;
    //     // buf.set_style(area, self.style);
    //     // buf.set_stringn(area.x, area.y, "▲", 1, Style::default());
    //     // buf.set_stringn(area.x, area.y + area.height, "▼", 1, Style::default());
    //
    //     // let range = self.max - self.min;
    //     // let value = self.default - self.min;
    //     // let width = area.width as f64;
    //     // let fill = (value as f64 / range as f64 * width).round() as u16;
    //     // for x in 0..fill {
    //     //     buf.set_stringn(area.x + x, area.y, "█", 1, Style::default());
    //     // }
    //     // for x in fill..area.width {
    //     //     buf.set_stringn(area.x + x, area.y, "░", 1, Style::default());
    //     // }
    // }
}
