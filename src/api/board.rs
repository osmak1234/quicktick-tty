use serde::{Deserialize, Serialize};

use crate::{app::App, helper::input::InputContentVariants, API_URL};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub uuid: String,
    pub name: String,
    pub user_uuid: String,
    pub special: Option<i32>,
}

pub async fn get_all_user_boards(app: &mut App) -> Option<Vec<Board>> {
    let client = &app.reqwest_client;
    let url = format!("{}/get/all_user_board", API_URL);
    //TODO: Error handling
    let response = client.get(&url).send().await.unwrap().text().await.unwrap();
    let boards: Vec<Board> = serde_json::from_str(&response).unwrap();
    Some(boards)
}

pub async fn create_board(app: &mut App) {
    let url = format!("{}/post/board", API_URL);
    let board: Board = match &app.input_content.variant {
        InputContentVariants::CreateBoard { name } => Board {
            uuid: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            user_uuid: app.user.as_ref().unwrap().uuid.clone(),
            special: None,
        },
        _ => {
            return;
        }
    };

    app.boards.items.push(board.clone());

    let board_json = serde_json::to_string(&board).unwrap();

    let client = app.reqwest_client.clone();
    // Spawn the async task
    tokio::spawn(async move {
        let _response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(board_json)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}

pub async fn delete_board(app: &mut App) {
    let board_uuid = app.boards.selected().unwrap().uuid.clone();

    app.boards.items.retain(|board| board.uuid != board_uuid);

    let client = app.reqwest_client.clone();

    // Spawn the async task
    tokio::spawn(async move {
        let url = format!("{}/delete/board/{}", API_URL, board_uuid);

        let _response = client
            .delete(&url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    });
}
