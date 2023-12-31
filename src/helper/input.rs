use crossterm::event::{Event, KeyEvent};
use tui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};
use tui_input::{backend::crossterm::EventHandler, Input};

pub const WINDOWN_WIDTH: usize = 47;
pub const SMALL_INPUT: usize = 1;
pub const BIG_INPUT: usize = 4;

pub const BACKSPACE_KEY_EVENT: Event = Event::Key(KeyEvent {
    code: crossterm::event::KeyCode::Backspace,
    modifiers: crossterm::event::KeyModifiers::NONE,
    state: crossterm::event::KeyEventState::NONE,
    kind: crossterm::event::KeyEventKind::Press,
});

#[derive(Debug, Clone)]
pub struct InputContent {
    pub visible: bool,
    pub selected_input: usize,
    pub variant: InputContentVariants,
}

#[derive(Debug, Clone)]
pub enum InputContentVariants {
    CreateTask {
        name: Input,
        description: Input,
    },
    CreateBoard {
        name: Input,
    },
    UpdateTask {
        name: Input,
        description: Input,
    },
    LogIn {
        email: Input,
        password: Input,
    },
    SignUp {
        email: Input,
        password: Input,
        name: Input,
    },
    ChangeUsername {
        name: Input,
    },
}

pub enum DataForSubmit {
    CreateTask {
        name: String,
        description: String,
    },
    CreateBoard {
        name: String,
    },
    UpdateTask {
        name: String,
        description: String,
    },
    LogIn {
        email: String,
        password: String,
    },
    SignUp {
        email: String,
        password: String,
        name: String,
    },
    ChangeUsername {
        name: String,
    },
}

impl PartialEq for InputContentVariants {
    fn eq(&self, other: &Self) -> bool {
        match self {
            InputContentVariants::CreateTask { .. } => {
                matches!(other, InputContentVariants::CreateTask { .. })
            }
            InputContentVariants::CreateBoard { .. } => {
                matches!(other, InputContentVariants::CreateBoard { .. })
            }
            InputContentVariants::UpdateTask { .. } => {
                matches!(other, InputContentVariants::UpdateTask { .. })
            }
            InputContentVariants::LogIn { .. } => {
                matches!(other, InputContentVariants::LogIn { .. })
            }
            InputContentVariants::SignUp { .. } => {
                matches!(other, InputContentVariants::SignUp { .. })
            }
            InputContentVariants::ChangeUsername { .. } => {
                matches!(other, InputContentVariants::ChangeUsername { .. })
            }
        }
    }
}

fn line_break_text(text: String) -> String {
    let mut name_to_render = String::new();

    for (index, char) in text.chars().enumerate() {
        if index % WINDOWN_WIDTH == 0 && index != 0 {
            name_to_render.push('\n');
        }
        name_to_render.push(char);
    }
    name_to_render
}

impl InputContent {
    pub fn show(&mut self, want_to_show: InputContentVariants) {
        if PartialEq::eq(&self.variant, &want_to_show) {
            self.visible = !self.visible;
        } else {
            self.variant = want_to_show;
            self.visible = true;
        }
    }

    pub fn cursor_coordinates(&mut self) -> (u16, u16) // (x, y) x = len of current input, and y is based on selected_input and variant
    {
        let y: u16;
        match self.variant {
            InputContentVariants::CreateTask { .. } => match self.selected_input {
                0 => {
                    y = 1;
                }
                1 => {
                    if self.selected_input_len() <= WINDOWN_WIDTH {
                        y = 1;
                    } else if self.selected_input_len() >= WINDOWN_WIDTH
                        && self.selected_input_len() <= 2 * WINDOWN_WIDTH
                    {
                        y = 2;
                    } else if self.selected_input_len() >= 2 * WINDOWN_WIDTH
                        && self.selected_input_len() <= 3 * WINDOWN_WIDTH
                    {
                        y = 3;
                    } else {
                        y = 4;
                    }
                }
                _ => {
                    y = 1;
                }
            },
            InputContentVariants::UpdateTask { .. } => match self.selected_input {
                0 => {
                    y = 1;
                }
                1 => {
                    if self.selected_input_len() <= WINDOWN_WIDTH {
                        y = 1;
                    } else if self.selected_input_len() >= WINDOWN_WIDTH
                        && self.selected_input_len() <= 2 * WINDOWN_WIDTH
                    {
                        y = 2;
                    } else if self.selected_input_len() >= 2 * WINDOWN_WIDTH
                        && self.selected_input_len() <= 3 * WINDOWN_WIDTH
                    {
                        y = 3;
                    } else {
                        y = 4;
                    }
                }
                _ => {
                    y = 1;
                }
            },
            _ => {
                y = 1;
            }
        }
        ((self.selected_input_len() + 1).try_into().unwrap(), y)
    }

    pub fn ui_to_render(&mut self) -> Vec<Paragraph> {
        let mut to_render = Vec::new();
        match &self.variant {
            InputContentVariants::CreateTask { name, description } => {
                // add \n after every 48 chars
                to_render.push(
                    Paragraph::new(name.to_string().clone())
                        .block(
                            Block::default()
                                .title("New task Name")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 0 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );

                to_render.push(
                    Paragraph::new(line_break_text(description.to_string().clone()))
                        .block(
                            Block::default()
                                .title("Description (optinal)")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 1 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
            InputContentVariants::CreateBoard { name } => {
                to_render.push(
                    Paragraph::new(name.to_string().clone())
                        .block(
                            Block::default()
                                .title("New board Name")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 0 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
            InputContentVariants::UpdateTask { name, description } => {
                to_render.push(
                    Paragraph::new(name.to_string().clone())
                        .block(
                            Block::default()
                                .title("Update task Name")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 0 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
                to_render.push(
                    Paragraph::new(line_break_text(description.to_string().clone()))
                        .block(
                            Block::default()
                                .title("Update task Description")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 1 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
            InputContentVariants::LogIn { email, password } => {
                to_render.push(
                    Paragraph::new(email.to_string().clone())
                        .block(
                            Block::default()
                                .title("Log in Email")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 0 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
                to_render.push(
                    Paragraph::new(password.to_string().clone())
                        .block(
                            Block::default()
                                .title("password")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 1 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
            InputContentVariants::SignUp {
                email,
                password,
                name,
            } => {
                to_render.push(
                    Paragraph::new(email.to_string().clone()).block(
                        Block::default()
                            .title("Sign up Email")
                            .borders(Borders::ALL)
                            .border_type(if self.selected_input == 0 {
                                BorderType::Double
                            } else {
                                BorderType::Plain
                            })
                            .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                    ),
                );
                to_render.push(
                    Paragraph::new(password.to_string().clone())
                        .block(
                            Block::default()
                                .title("Name")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 1 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
                to_render.push(
                    Paragraph::new(name.to_string().clone())
                        .block(
                            Block::default()
                                .title("Password (will be encrypted)")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 2 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
            InputContentVariants::ChangeUsername { name } => {
                to_render.push(
                    Paragraph::new(name.to_string().clone())
                        .block(
                            Block::default()
                                .title("Change username")
                                .borders(Borders::ALL)
                                .border_type(if self.selected_input == 0 {
                                    BorderType::Double
                                } else {
                                    BorderType::Plain
                                }),
                        )
                        .style(Style::default().bg(Color::Indexed(235)).fg(Color::White)),
                );
            }
        };
        to_render
    }

    pub fn data_for_submit(&mut self) -> Option<DataForSubmit> {
        match &self.variant {
            InputContentVariants::CreateTask { name, description } => {
                Some(DataForSubmit::CreateTask {
                    name: name.to_string(),
                    description: description.to_string(),
                })
            }
            InputContentVariants::CreateBoard { name } => Some(DataForSubmit::CreateBoard {
                name: name.to_string(),
            }),
            InputContentVariants::UpdateTask { name, description } => {
                Some(DataForSubmit::UpdateTask {
                    name: name.to_string(),
                    description: description.to_string(),
                })
            }
            InputContentVariants::LogIn { email, password } => Some(DataForSubmit::LogIn {
                email: email.to_string(),
                password: password.to_string(),
            }),
            InputContentVariants::SignUp {
                email,
                password,
                name,
            } => Some(DataForSubmit::SignUp {
                email: email.to_string(),
                password: password.to_string(),
                name: name.to_string(),
            }),
            InputContentVariants::ChangeUsername { name } => Some(DataForSubmit::ChangeUsername {
                name: name.to_string(),
            }),
        }
    }

    pub fn input_count(&self) -> i32 {
        match self.variant {
            InputContentVariants::CreateTask { .. } => 2,
            InputContentVariants::CreateBoard { .. } => 1,
            InputContentVariants::UpdateTask { .. } => 2,
            InputContentVariants::LogIn { .. } => 2,
            InputContentVariants::SignUp { .. } => 3,
            InputContentVariants::ChangeUsername { .. } => 1,
        }
    }

    pub fn cycle_selected_input(&mut self) {
        self.selected_input += 1;
        if self.selected_input >= self.input_count() as usize {
            self.selected_input = 0;
        }
    }

    pub fn cycle_selected_input_backwards(&mut self) {
        if self.selected_input == 0 {
            self.selected_input = self.input_count() as usize - 1;
        } else {
            self.selected_input -= 1;
        }
    }

    pub fn handle_keypress(&mut self, key_event: KeyEvent) {
        let key_event = Event::Key(key_event);
        match self.variant {
            InputContentVariants::CreateTask {
                ref mut name,
                ref mut description,
            } => match self.selected_input {
                0 => {
                    if name.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        name.handle_event(&key_event);
                    }
                }
                1 => {
                    if description.to_string().len() < WINDOWN_WIDTH * BIG_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        description.handle_event(&key_event);
                    }
                }
                _ => {}
            },
            InputContentVariants::CreateBoard { ref mut name } => {
                if let 0 = self.selected_input {
                    if name.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        name.handle_event(&key_event);
                    }
                }
            }
            InputContentVariants::UpdateTask {
                ref mut name,
                ref mut description,
            } => match self.selected_input {
                0 => {
                    if name.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        name.handle_event(&key_event);
                    }
                }
                1 => {
                    if description.to_string().len() < WINDOWN_WIDTH * BIG_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        description.handle_event(&key_event);
                    }
                }
                _ => {}
            },
            InputContentVariants::LogIn {
                ref mut email,
                ref mut password,
            } => match self.selected_input {
                0 => {
                    if email.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        email.handle_event(&key_event);
                    }
                }
                1 => {
                    if password.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        password.handle_event(&key_event);
                    }
                }
                _ => {}
            },
            InputContentVariants::SignUp {
                ref mut email,
                ref mut password,
                ref mut name,
            } => match self.selected_input {
                0 => {
                    if email.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        email.handle_event(&key_event);
                    }
                }
                1 => {
                    if password.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        password.handle_event(&key_event);
                    }
                }
                2 => {
                    if name.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT {
                        name.handle_event(&key_event);
                    }
                }
                _ => {}
            },
            InputContentVariants::ChangeUsername { ref mut name } => {
                if let 0 = self.selected_input {
                    if name.to_string().len() < WINDOWN_WIDTH * SMALL_INPUT
                        || key_event == BACKSPACE_KEY_EVENT
                    {
                        name.handle_event(&key_event);
                    }
                }
            }
        }
    }

    pub fn selected_input_len(&mut self) -> usize {
        match self.variant {
            InputContentVariants::CreateTask {
                ref name,
                ref description,
            } => {
                if self.selected_input == 0 {
                    name.to_string().chars().count()
                } else {
                    description.to_string().chars().count()
                }
            }
            InputContentVariants::CreateBoard { ref name } => name.to_string().chars().count(),
            InputContentVariants::UpdateTask {
                ref name,
                ref description,
            } => {
                if self.selected_input == 0 {
                    name.to_string().chars().count()
                } else {
                    description.to_string().chars().count()
                }
            }
            InputContentVariants::LogIn {
                ref email,
                ref password,
            } => {
                if self.selected_input == 0 {
                    email.to_string().chars().count()
                } else {
                    password.to_string().chars().count()
                }
            }
            InputContentVariants::SignUp {
                ref email,
                ref password,
                ref name,
            } => {
                if self.selected_input == 0 {
                    email.to_string().chars().count()
                } else if self.selected_input == 1 {
                    password.to_string().chars().count()
                } else {
                    name.to_string().chars().count()
                }
            }
            InputContentVariants::ChangeUsername { ref name } => name.to_string().chars().count(),
        }
    }
}

impl Default for InputContent {
    fn default() -> Self {
        Self {
            visible: false,
            selected_input: 0,
            variant: InputContentVariants::CreateTask {
                name: Input::default(),
                description: Input::default(),
            },
        }
    }
}
