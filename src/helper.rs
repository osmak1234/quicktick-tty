use tui::widgets::ListState;

use crate::{
    api::user::{sign_in, Credentials},
    app::App,
};
use tui_input::Input;

pub async fn refetch_data(app: &mut App) {
    app.credentials = Some(Credentials {
        email: "cookie".to_string(),
        password: "cookie".to_string(),
    });

    sign_in(app, app.credentials.clone().unwrap()).await;
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
pub struct InputState {
    pub visible: bool,
    pub value: Input,
    pub input_mode: InputMode,
}

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn selected(&self) -> Option<T>
    where
        T: Clone,
    {
        self.state.selected().map(|i| self.items[i].clone())
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
