use std::io::Stderr;

use crate::api::user::sign_in;
use crate::app::{App, AppResult};
use crate::helper::input::InputContentVariants;
use crate::tui::Tui;
use crossterm::event::{
    // Event,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use tui::prelude::CrosstermBackend;
// use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    terminal: &mut Tui<CrosstermBackend<Stderr>>,
) -> AppResult<()> {
    match app.input_content.visible {
        true => match key_event.code {
            KeyCode::Esc => {
                app.input_content.visible = false;
                app.input_content.selected_input = 0;
                terminal.hide_cursor()?;
            }

            KeyCode::Tab => {
                app.input_content.cycle_selected_input();
            }

            KeyCode::Enter => {
                match &app.input_content.variant {
                    InputContentVariants::LogIn { email, password } => {
                        let credentials = crate::api::user::Credentials {
                            email: email.to_string(),
                            password: password.to_string(),
                        };
                        let user_data = sign_in(app, credentials).await;
                        if let Some(user) = user_data {
                            app.user = Some(user);
                            app.input_content.visible = false;
                            app.input_content.selected_input = 0;
                            terminal.hide_cursor()?;
                        }
                    }
                    InputContentVariants::SignUp { .. } => {
                        let user_data = crate::api::user::sign_up(app).await;
                        if user_data.is_ok() {
                            app.input_content.visible = false;
                            app.input_content.selected_input = 0;
                            terminal.hide_cursor()?;
                        }
                    }
                    _ => match app.selected_widget {
                        true => {
                            crate::api::board::create_board(app).await;
                        }
                        false => {
                            crate::api::task::create_task(app).await;
                        }
                    },
                }
                app.input_content.visible = false;
                app.input_content.selected_input = 0;
                terminal.hide_cursor()?;
            }

            _ => app.input_content.handle_keypress(key_event),
        },
        false => {
            match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }

                KeyCode::Char('a') => {
                    app.input_content.show(InputContentVariants::LogIn {
                        email: Input::default(),
                        password: Input::default(),
                    });
                }

                KeyCode::Char('A') => {
                    app.input_content.show(InputContentVariants::SignUp {
                        email: Input::default(),
                        name: Input::default(),
                        password: Input::default(),
                    });
                }

                KeyCode::Char('l') | KeyCode::Char('h') => {
                    app.selected_widget = !app.selected_widget;
                }

                KeyCode::Char('k') => {
                    if app.selected_widget {
                        app.boards.previous();
                    } else {
                        app.tasks.previous();
                    }
                }

                KeyCode::Char('j') => {
                    if app.selected_widget {
                        app.boards.next();
                    } else {
                        app.tasks.next();
                    }
                }

                KeyCode::Char(' ') => {
                    if app.selected_widget {
                    } else {
                        crate::api::task::toggle_task(app).await;
                    }
                }

                KeyCode::Char('r') => {
                    crate::helper::functions::refetch_data(app).await;
                }

                KeyCode::Char('i') => {
                    terminal.show_cursor()?;
                    match app.selected_widget {
                        true => app.input_content.show(InputContentVariants::CreateBoard {
                            name: Input::default(),
                        }),
                        false => app.input_content.show(InputContentVariants::CreateTask {
                            name: Input::default(),
                            description: Input::default(),
                        }),
                    }
                }

                KeyCode::Char('d') => {
                    if app.selected_widget {
                        crate::api::board::delete_board(app).await;
                    } else {
                        crate::api::task::delete_task(app).await;
                    }
                }

                _ => {}
            }
        }
    }
    Ok(())
}
