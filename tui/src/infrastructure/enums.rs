use color_eyre::eyre;
use ratatui::crossterm::event::{KeyEvent, MouseEvent};
use ratatui::Frame;
use ratatui::prelude::Rect;
// use crossterm::{
//     event::{
//         DisableBracketedPaste,
//         DisableMouseCapture,
//         EnableBracketedPaste,
//         EnableMouseCapture,
//         Event as
//         CEvent,
//         EventStream,
//         KeyEvent,
//         KeyEventKind,
//         MouseEvent
//     }
// };
use serde::{Deserialize, Serialize};
use strum::Display;
use crate::components::about::About;
use crate::components::main::MainTUI;
use crate::components::quit::Quit;
use crate::components::settings::Settings;

// Think in terms of "screens" for the TUI. The TUI can be in one of the following screens:
// - Main screen: The default screen. The user can view their playlists, tracks, and albums.
// - Playlist screen: The user can view the tracks in a playlist. The user can switch back to
//   the main screen by pressing the 'Esc' key.
// - Album screen: The user can view the tracks in an album. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Track screen: The user can view the details of a track. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Artist screen: The user can view the tracks by an artist. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Search screen: The user can search for items in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Visual screen: The user can view data regarding their listening habits. The user can switch
//   back to the main screen by pressing the 'Esc' key.
// - Select screen: The user can select items in the TUI. The user can switch back to the main
//   screen by pressing the 'Esc' key.
// - Command-line screen: The user can enter commands in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Settings screen: The user can view and modify the settings of the TUI. The user can switch
//   back to the main screen by pressing the 'Esc' key.
// - Help screen: The user can view the help information for the TUI. The user can switch back to
//   the main screen by pressing the 'Esc' key.
// - About screen: The user can view information about the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Quit screen: The user can quit the TUI. The user can switch back to the main screen by pressing
//   the 'Esc' key.
// - Error screen: The user can view error messages in the TUI. The user can switch back to the main
//   screen by pressing the 'Esc' key.
// - Loading screen: The user can view loading messages in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Confirmation screen: The user can confirm actions in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Warning screen: The user can view warning messages in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Success screen: The user can view success messages in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Failure screen: The user can view failure messages in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Info screen: The user can view information messages in the TUI. The user can switch back to the
//   main screen by pressing the 'Esc' key.
// - Debug screen: The user can view debug messages in the TUI. The user can switch back to the main
//   screen by pressing the 'Esc' key.
// - Informatics screen: The user can view informatics messages in the TUI. The user can switch back
//   to the main screen by pressing the 'Esc' key.
// - Updates screen: The user can view updates messages in the TUI. The user can switch back to the main
//   screen by pressing the 'Esc' key.

#[derive(Default, Clone, Debug, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ScreenState {
    #[default]
    Main,
    // Playlist,
    // Album,
    // Track,
    // Artist,
    // Search,
    // Visual,
    // Select,
    // CommandLine,
    Settings,
    // Help,
    About,
    Quit,
    // Error,
    // Loading,
    // Confirmation,
    // Warning,
    // Success,
    // Failure,
    // Info,
    // Debug,
    // Informatics,
    // Updates,
}

impl ScreenState {
    pub fn render(&mut self, frame: &mut Frame, open_screen: &mut OpenScreen, area: Rect) -> eyre::Result<()> {
        match self {
            // CurrentScreen::Home => Home::render(open_screen.home(), frame),
            ScreenState::Main => {
                if let OpenScreen::Main(main) = open_screen {
                    main.render(frame).expect("Main page failed to render");
                };
                Ok(())
            }
            ScreenState::Quit => Quit::render(&mut Quit::new(), frame),
            ScreenState::About => {
                if let OpenScreen::About(about) = open_screen {
                    about.render(frame, area).expect("About page failed to render");
                };
                Ok(())
            },
            ScreenState::Settings => {
                if let OpenScreen::Settings(settings) = open_screen {
                    settings.render(frame, area).expect("Settings page failed to render");
                };
                Ok(())
            }
        }
    }
}
#[derive(Clone, Debug, Default)]
pub enum CurrentScreen {
    #[default]
    About,
    CurrentlyPlaying,
    Playlists,
    ReleaseRadar,
    Settings,
}

impl CurrentScreen {
    #[allow(unused)]
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        match self {
            CurrentScreen::About => {
                let mut about = About::new();
                about.render(frame, area).expect("About page failed to render");
            }
            CurrentScreen::CurrentlyPlaying => {
                // let currently_playing = CurrentlyPlaying::new();
                // currently_playing.render(frame).expect("Currently Playing page failed to render");
            }
            CurrentScreen::Playlists => {
                // let current_playlists = CurrentPlaylists::new();
                // current_playlists.render(frame).expect("Current Playlists page failed to render");
            }
            CurrentScreen::ReleaseRadar => {
                // let release_radar = ReleaseRadar::new();
                // release_radar.render(frame).expect("Release Radar page failed to render");
            }
            CurrentScreen::Settings => {
                let mut settings = Settings::new();
                settings.render(frame, area).expect("Settings page failed to render");
            }
        }
    }

}
pub enum OpenScreen {
    Main(MainTUI),
    About(About),
    #[allow(unused)]
    Quit(Quit),
    CurrentPlaylists,
    ReleaseRadar,
    Settings(Settings),
}
#[allow(unused)]
pub enum Interactions {
    Hover,
    Click,
    TextEntered
    
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Event {
    Init,
    Quit,
    Error,
    Closed,
    Tick,
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    ClearScreen,
    Error(String),
    Help,
    None,
}
#[derive(Clone, Debug)]
pub enum SettingType {
    OnOff,
    #[allow(unused)]
    Int,
    Category,
    Slider,
    #[allow(unused)]
    String,
}
