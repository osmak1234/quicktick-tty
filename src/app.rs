use crate::api::board::Board;
use crate::api::task::Task;
use crate::api::user::{Credentials, User};
use crate::helper::input::InputContent;
use crate::helper::stateful_list::StatefulList;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub reqwest_client: reqwest::Client,
    pub running: bool,
    pub user: Option<User>,
    pub credentials: Option<Credentials>,
    pub boards: StatefulList<Board>,
    pub tasks: StatefulList<Task>,
    pub task_data: Vec<Task>,
    pub selected_widget: bool,
    pub input_content: InputContent,
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
            running: true,
            user: None,
            credentials: Some(Credentials {
                email: "".to_string(),
                password: "".to_string(),
            }),
            task_data: Vec::new(),
            tasks: StatefulList::with_items(Vec::new()),
            boards: StatefulList::with_items(Vec::new()),
            selected_widget: false,
            input_content: InputContent::default(),
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
