use tui_input::Input;

use crate::api::board::Board;
use crate::api::task::Task;
use crate::api::user::{Credentials, User};
use crate::helper::{InputMode, InputState, StatefulList};
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub reqwest_client: reqwest::Client,
    pub api_url: String,
    pub running: bool,
    pub user: Option<User>,
    pub credentials: Option<Credentials>,
    pub boards: StatefulList<Board>,
    pub tasks: StatefulList<Task>,
    pub selected_widget: bool,
    pub input: InputState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            reqwest_client: reqwest::ClientBuilder::new()
                .user_agent("quicktick-tty")
                .cookie_store(true)
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .unwrap(),
            api_url: String::from("https://quicktick-api.fly.dev"),

            running: true,
            user: None,
            credentials: Some(Credentials {
                email: "tadeashanus31@gmail.com".to_string(),
                password: "123456".to_string(),
            }),
            tasks: StatefulList::with_items(Vec::new()),
            boards: StatefulList::with_items(Vec::new()),
            selected_widget: false,
            input: InputState {
                value: Input::default(),
                input_mode: InputMode::Editing,
                visible: false,
            },
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
