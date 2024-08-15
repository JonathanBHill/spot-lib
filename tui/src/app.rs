use std::io;

use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

use spot_lib_general::playlists::releaseradar::Release;

use crate::components::main::MainTUI;
use crate::components::settings::Settings;
use crate::infrastructure::enums::{OpenScreen, ScreenState};
use crate::ui::ui;

pub struct Spotui {
    #[allow(unused)]
    title: String,
    #[allow(unused)]
    message: String,
    pub(crate) current_screen: ScreenState,
    pub(crate) open_screen: OpenScreen,
    pub layout: Layout,
    selected_tab: usize,
    home: MainTUI,
    #[allow(unused)]
    settings: Settings,
    exit: bool,
}

impl Spotui {
    pub fn new(release_radar: Release) -> Self {
        let layout = Layout::default()
        .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),  // Navigation tabs
                    Constraint::Fill(1),  // Main area
                    Constraint::Length(3),  // Navigation legend
                ]
                    .as_ref(),
            );
        let main = MainTUI::new(layout.clone(), release_radar);
        Spotui {
            title: "Spotui Title Property".to_string(),
            message: "Spotui Message Property".to_string(),
            current_screen: ScreenState::Main,
            open_screen: OpenScreen::Main(main.clone()),
            layout,
            selected_tab: 0,
            home: main,
            settings: Settings::new(),
            exit: false,
        }
    }
    // pub fn next_tab(&mut self) -> anyhow::Result<()> {
    //     self.selected_tab = (self.selected_tab + 1) % self.tab_labels.len();
    //     self.current_screen = match self.selected_tab {
    //         0 => CurrentScreen::About,
    //         1 => CurrentScreen::CurrentlyPlaying,
    //         2 => CurrentScreen::Playlists,
    //         3 => CurrentScreen::ReleaseRadar,
    //         4 => CurrentScreen::Settings,
    //         _ => CurrentScreen::About,
    //     };
    //     Ok(())
    // }
  pub fn set_open_screen(&mut self, tab: usize) -> anyhow::Result<()> {
        self.selected_tab = tab;
        
        self.open_screen = match self.home.selected_tab {
            0 => OpenScreen::About(self.home.about.clone()),
            1 => OpenScreen::CurrentPlaylists,
            3 => OpenScreen::ReleaseRadar,
            4 => OpenScreen::Settings(Settings::new()),
            _ => OpenScreen::About(self.home.about.clone()),
        };
        Ok(())
    }
    
    // pub fn previous_tab(&mut self) {
    //     if self.selected_tab > 0 {
    //         self.selected_tab -= 1;
    //     } else {
    //         self.selected_tab = self.tab_labels.len() - 1;
    //     }
    // }
    #[allow(unused)]
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    #[allow(unused)]
    fn handle_key_event(&mut self, _key_event: KeyEvent) {
        ()
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
// async fn run_ui<'a>(f: &mut Frame<'a>, app: &mut Spotui) -> io::Result<()> {
    // Perform async tasks here, like fetching data or awaiting network calls
    // Example: fetch data, then pass it to the ui function
    // let data = fetch_data().await?;
    
    // Call the ui function after all async tasks are done
    // ui(f, app).await;
    // Ok(())
// }
pub fn run_app2<B: Backend>(terminal: &mut Terminal<B>, app: &mut Spotui) -> io::Result<()> {
    // let async_ui_task = async {
    //     terminal.draw(|f| {
    //         futures::executor::block_on(run_ui(f, app)).expect("UI did not load properly");
    //     })?;
    //     Ok::<(), io::Error>(())
    // };
    // futures::executor::block_on(async_ui_task)?;
    loop {
        terminal.draw(|f| {
            // let ui = ui(f, app);
            ui(f, app).expect("UI did not load properly");
        })?;
        
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match &app.current_screen {
                ScreenState::Main => match key.code {
                    KeyCode::Char('q') => {
                        app.current_screen = ScreenState::Quit;
                    },
                    KeyCode::Enter => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.settings.edit_setting();
                        }
                    },
                    KeyCode::Left => {
                        // let mut tab: usize = 0;
                        
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                ()
                            } else {
                                home.previous_tab();
                                // tab = home.selected_tab;
                            }
                        }
                    },
                    KeyCode::Right => {
                        // let mut tab: usize = 0;
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                ()
                            } else {
                                home.next_tab();
                                // tab = home.selected_tab;
                            }
                        }
                    },
                    KeyCode::Tab => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.next_tab();
                        }
                    },
                    
                    KeyCode::Char('j') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                ()
                            } else {
                                home.settings.next_item();
                            }
                        }
                    },
                    KeyCode::Char('k') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                ()
                            } else {
                                home.settings.previous_item();
                            }
                        }
                    },
                    KeyCode::Char('h') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                home.settings.active_slider.decrease();
                            } else {
                                home.settings.previous_group();
                            }
                        }
                    },
                    KeyCode::Char('l') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            if home.settings.popup_active {
                                home.settings.active_slider.increase();
                            } else {
                                home.settings.next_group();
                            }
                        }
                    },
                    KeyCode::Char('a') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(0);
                        }
                    },
                    KeyCode::Char('1') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(0);
                        }
                    },
                    KeyCode::Char('c') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(1);
                        }
                    },
                    KeyCode::Char('2') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(1);
                        }
                    },
                    KeyCode::Char('p') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(2);
                        }
                    },
                    KeyCode::Char('3') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(2);
                        }
                    },
                    KeyCode::Char('r') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(3);
                        }
                    },
                    KeyCode::Char('4') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(3);
                        }
                    },
                    KeyCode::Char('s') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(4);
                        }
                    },
                    KeyCode::Char('5') => {
                        if let OpenScreen::Main(ref mut home) = app.open_screen {
                            home.set_tab(4);
                        }
                    },
                    KeyCode::Esc => {
                        app.current_screen = ScreenState::Quit;
                    },
                    _ => {}
                },
                ScreenState::Quit => match key.code {
                        KeyCode::Char('y') => {
                            app.exit();
                            return Ok(())
                        },
                        KeyCode::Char('n') => {
                            app.current_screen = ScreenState::Main;
                        },
                        KeyCode::Esc => {
                            app.current_screen = ScreenState::Main;
                        },
                        KeyCode::Enter => {
                            app.exit();
                            return Ok(())
                        },
                        _ => {}
                },
                _ => {}
            }
            
        }
    }
}
