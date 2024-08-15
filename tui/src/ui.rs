use color_eyre::eyre;
use ratatui::{
    Frame
    
    
    ,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::Spotui;

pub fn ui<'a>(f: &mut Frame<'a>, app: &mut Spotui) -> eyre::Result<()> {
    // match app.current_screen {
    //     ScreenState::Main => {
    //         app.current_screen.render(f, &mut app.open_screen, app.layout.split(f.size())[1]).await.expect("panic message");
    //         Ok(())
    //     },
    //     _ => Err(()),
    // }
    app.current_screen.render(f, &mut app.open_screen, app.layout.split(f.size())[1])?;
    Ok(())
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
#[allow(dead_code)]
pub(crate) fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    
    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
