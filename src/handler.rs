use std::io::Stderr;

use crate::api::user::sign_in;
use crate::app::{App, AppResult};
use crate::helper::InputMode;
use crate::tui::Tui;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui::prelude::CrosstermBackend;
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    terminal: &mut Tui<CrosstermBackend<Stderr>>,
) -> AppResult<()> {
    match app.input.visible {
        true => match app.input.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => match key_event.code {
                KeyCode::Enter => {
                    app.input.visible = false;
                    crate::api::task::create_task(app).await;
                    app.input.value.reset();
                }
                KeyCode::Esc => {
                    app.input.visible = false;
                }

                _ => {
                    app.input.value.handle_event(&Event::Key(key_event));
                    if app.input.value.to_string().len() > 57 {
                        app.input.value.handle_event(&Event::Key(KeyEvent {
                            code: KeyCode::Backspace,
                            modifiers: KeyModifiers::empty(),
                            kind: key_event.kind,
                            state: key_event.state,
                        }));
                    }
                }
            },
        },
        false => {
            match key_event.code {
                // Exit application on `ESC` or `q`
                KeyCode::Char('q') => {
                    app.quit();
                }
                // Exit application on `Ctrl-C`
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    if key_event.modifiers == KeyModifiers::CONTROL {
                        app.quit();
                    }
                }

                KeyCode::Char('a') | KeyCode::Char('A') => {
                    let data = sign_in(app, app.credentials.clone().unwrap()).await;

                    let maybe_user = data.unwrap();
                    app.user = Some(maybe_user);
                }

                KeyCode::Char('l')
                | KeyCode::Char('L')
                | KeyCode::Char('h')
                | KeyCode::Char('H') => {
                    app.selected_widget = !app.selected_widget;
                }

                KeyCode::Char('k') | KeyCode::Char('K') => {
                    if app.selected_widget {
                        app.boards.previous();
                    } else {
                        app.tasks.previous();
                    }
                }

                KeyCode::Char('j') | KeyCode::Char('J') => {
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

                KeyCode::Char('r') | KeyCode::Char('R') => {
                    crate::helper::refetch_data(app).await;
                }

                KeyCode::Char('i') | KeyCode::Char('I') => {
                    terminal.show_cursor()?;
                    app.input.visible = true;
                    app.input.input_mode = InputMode::Editing;
                }

                _ => {}
            }
        }
    }
    Ok(())
}
