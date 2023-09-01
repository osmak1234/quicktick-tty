use serde::{Deserialize, Serialize};

use crate::app::App;

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
    let url = format!("{}/get/all_user_tasks", app.api_url);
    let response = client.get(&url).send().await.unwrap().text().await.unwrap();
    let tasks: Vec<Task> = serde_json::from_str(&response).unwrap();
    Some(tasks)
}

pub async fn create_task(app: &mut App) {
    let new_task = Task {
        uuid: uuid::Uuid::new_v4().to_string(),
        name: app.input.value.to_string(),
        description: "".to_string(),
        completed: false,
        user_uuid: app.user.as_ref().unwrap().uuid.clone(),
        board_uuid: app.boards.items[app.boards.state.selected().unwrap()]
            .uuid
            .clone(),
    };

    app.tasks.items.push(new_task.clone());

    let client = &app.reqwest_client;
    let url = format!("{}/post/create_task", app.api_url);
    //TODO: Error handling
    let _response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(format!(
            "{{\"name\": \"{}\", \"description\": \"{}\", \"user_uuid\": \"{}\", \"board_uuid\": \"{}\"}}",
            new_task.name,
            "",
            new_task.user_uuid,
            new_task.board_uuid
        ))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}

pub async fn toggle_task(app: &mut App) {
    let client = &app.reqwest_client;

    app.tasks.items[app.tasks.state.selected().unwrap()].completed =
        !app.tasks.items[app.tasks.state.selected().unwrap()].completed;

    let task_uuid = app.tasks.items[app.tasks.state.selected().unwrap()]
        .uuid
        .clone();

    let url = format!("{}/patch/task", app.api_url,);

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
}
