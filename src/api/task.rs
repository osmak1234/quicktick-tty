use core::panic;

use serde::{Deserialize, Serialize};

use crate::{
    app::App,
    helper::input::{InputContent, InputContentVariants},
    API_URL,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub user_uuid: String,
    pub board_uuid: String,
}
pub async fn get_all_tasks(app: &mut App) -> Option<Vec<Task>> {
    let client = &app.reqwest_client;
    let url = format!("{}/get/all_user_tasks?device_identifier=tty", API_URL);
    let response = client.get(&url).send().await.unwrap().text().await.unwrap();
    let tasks: Vec<Task> = serde_json::from_str(&response).unwrap();
    Some(tasks)
}

pub async fn delete_task(app: &mut App) {
    let task_uuid = app.tasks.selected().unwrap().clone().uuid;
    let board_special = app.boards.selected().unwrap().special;
    if board_special.is_some_and(|special| special == 1) || board_special.is_none() {
        app.task_data.iter_mut().for_each(|task| {
            if task.uuid == task_uuid {
                task.board_uuid = board_special.unwrap().to_string();
            }
        });

        let special_2_board_uuid = app
            .boards
            .items
            .iter()
            .find(|board| board.special == Some(2))
            .unwrap()
            .uuid
            .clone();
        patch_task(app, Action::MoveBoard(special_2_board_uuid)).await;
    } else if board_special.unwrap() == 2 {
        app.task_data.retain(|task| task.uuid != task_uuid);

        let client = app.reqwest_client.clone();

        // Spawn the async task
        tokio::spawn(async move {
            let url = format!(
                "{}/delete/task/{}?device_identifier=tty",
                API_URL, task_uuid
            );

            //TODO: Error handling
            let response = client
                .delete(&url)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        });
    }
}

pub async fn create_task(app: &mut App) {
    let new_task: Task = match &app.input_content.variant {
        InputContentVariants::CreateTask { name, description } => Task {
            uuid: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: description.to_string(),
            completed: false,
            user_uuid: app.user.as_ref().unwrap().uuid.clone(),
            board_uuid: app.boards.selected().unwrap().uuid.clone(),
        },
        _ => {
            return;
        }
    };

    app.task_data.push(new_task.clone());

    let url = format!("{}/post/create_task?device_identifier=tty", API_URL);
    let client = app.reqwest_client.clone();

    // Spawn the async task
    tokio::spawn(async move {
        let _response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&new_task).unwrap())
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}

pub async fn move_task(_app: &mut App) {
    todo!()
}

pub async fn toggle_task(app: &mut App) {
    // Clone the values you need

    let selected_task_index = app.tasks.state.selected().unwrap_or(9999);
    if selected_task_index == 9999 {
        return;
    }
    if app.tasks.items.len() < selected_task_index || app.tasks.items.is_empty() {
        return;
    }

    let task_uuid = app.tasks.items[selected_task_index].uuid.clone();

    // Toggle the completed field
    app.task_data.iter_mut().for_each(|task| {
        if task.uuid == task_uuid {
            task.completed = !task.completed;
        }
    });

    let client = app.reqwest_client.clone();

    // Spawn the async task
    tokio::spawn(async move {
        let url = format!("{}/patch/task?device_identifier=tty", API_URL);

        //TODO: Error handling
        let _response = client
            .patch(&url)
            .header("Content-Type", "application/json")
            .body(format!(
                "{{\"task_uuid\": \"{}\", \"action\": \"ToggleTask\"}}",
                task_uuid
            ))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}

pub enum Action {
    RenameTask(String),
    ChangeDesc(String),
    ToggleTask,
    ChangeOrder(i32),
    MoveBoard(String),
}

pub async fn patch_task(app: &mut App, action: Action) {
    if app.tasks.selected().is_none() {
        return;
    }

    let task_uuid = app.tasks.selected().unwrap().uuid.clone();

    let client = app.reqwest_client.clone();

    let body = match action {
        Action::RenameTask(_) => todo!(),
        Action::ChangeDesc(_) => todo!(),
        Action::ToggleTask => {
            format!(
                "{{\"task_uuid\": \"{}\", \"action\": \"ToggleTask\"}}",
                task_uuid.clone()
            )
        }
        Action::ChangeOrder(_) => todo!(),
        Action::MoveBoard(board_uuid) => {
            format!(
                "{{\"task_uuid\": \"{}\", \"action\": \"MoveBoard\", \"board_uuid\": \"{}\"}}",
                task_uuid, board_uuid
            )
        }
    };

    // Spawn the async task
    tokio::spawn(async move {
        let url = format!("{}/patch/task?device_identifier=tty", API_URL);

        let _response = client
            .patch(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}
