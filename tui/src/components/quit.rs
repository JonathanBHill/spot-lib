use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

use crate::components::popups;
use crate::config::Config;

#[derive(Default, Clone, Debug)]
pub struct Quit {
    #[allow(unused)]
    config: Config,
    // popup_active: bool,
}

impl Quit {
    pub fn new() -> Self {
        let quit = Self::default();
        // quit.popup_active = true;
        quit
    }
    
    // pub fn toggle_input_popup(&mut self) {
    //     self.popup_active = !self.popup_active;
    // }
    pub fn render(&mut self, frame: &mut Frame) -> Result<()> {
        // frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
        // let popup_block = Block::default()
        //     .title("Y/N")
        //     .borders(Borders::NONE)
        //     .style(Style::default().bg(Color::DarkGray));
        //
        // let exit_text = Text::styled(
        //     "Would you like to output the buffer as json? (y/n)",
        //     Style::default().fg(Color::Red),
        // );
        // // the `trim: false` will stop the text from being cut off when over the edge of the block
        // let exit_paragraph = Paragraph::new(exit_text)
        //     .block(popup_block)
        //     .wrap(Wrap { trim: false });
        //
        // let area = crate::ui::centered_rect(60, 25, frame.size());
        // frame.render_widget(exit_paragraph, area);
        // frame.render_widget(Paragraph::new("This is the quit screen"), area);
        let popup_area = popups::centered_rect(60, 20, frame.size());
        let popup = Block::default()
            .title("Quit?")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .style(
                Style::default()
                    .bg(Color::Rgb(0,40,0))
                    .fg(Color::Rgb(205,255,205))
            );
        let popup_text = Paragraph::new("Would you like to exit the application? [y/n]")
            .style(Style::default().fg(Color::Rgb(205,255,205))).block(popup);
        frame.render_widget(popup_text, popup_area);

            // frame.render_widget(popup_text, popup_area);
        Ok(())
    }
}

// impl Comp for Quit {
//     fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
//         self.command_tx = Some(tx);
//         Ok(())
//     }
//
//     fn register_config_handler(&mut self, config: Config) -> Result<()> {
//         self.config = config;
//         Ok(())
//     }
//
//     fn update(&mut self, action: Action) -> Result<Option<Action>> {
//         match action {
//             Action::Tick => {
//                 // add any logic here that should run on every tick
//             }
//             Action::Render => {
//                 // add any logic here that should run on every render
//             }
//             _ => {}
//         }
//         Ok(None)
//     }
//
//     fn render(&mut self, frame: &mut Frame) -> Result<()> {
//         // frame.render_widget(Clear, frame.size()); //this clears the entire screen and anything already drawn
//         // let popup_block = Block::default()
//         //     .title("Y/N")
//         //     .borders(Borders::NONE)
//         //     .style(Style::default().bg(Color::DarkGray));
//         //
//         // let exit_text = Text::styled(
//         //     "Would you like to output the buffer as json? (y/n)",
//         //     Style::default().fg(Color::Red),
//         // );
//         // // the `trim: false` will stop the text from being cut off when over the edge of the block
//         // let exit_paragraph = Paragraph::new(exit_text)
//         //     .block(popup_block)
//         //     .wrap(Wrap { trim: false });
//         //
//         // let area = crate::ui::centered_rect(60, 25, frame.size());
//         // frame.render_widget(exit_paragraph, area);
//         // frame.render_widget(Paragraph::new("This is the quit screen"), area);
//         if self.popup_active {
//             let popup_area = popups::centered_rect(60, 20, frame.size());
//             let popup = Block::default()
//                 .title("Quit Menu")
//                 .borders(Borders::ALL)
//                 .style(Style::default().bg(Color::White).fg(Color::Black));
//             let popup_text = Paragraph::new("This is a popup!")
//                 .style(Style::default().fg(Color::Black)).block(popup);
//             frame.render_widget(popup_text, popup_area);
//
//             // frame.render_widget(popup_text, popup_area);
//         }
//         Ok(())
//     }
// }
