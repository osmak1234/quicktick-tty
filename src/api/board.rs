use serde::{Deserialize, Serialize};

use crate::app::App;

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub uuid: String,
    pub name: String,
    pub user_uuid: String,
    pub special: Option<i32>,
}

pub async fn get_all_user_boards(app: &mut App) -> Option<Vec<Board>> {
    let client = &app.reqwest_client;
    let url = format!("{}/get/all_user_board", app.api_url);
    //TODO: Error handling
    let response = client.get(&url).send().await.unwrap().text().await.unwrap();
    let boards: Vec<Board> = serde_json::from_str(&response).unwrap();
    Some(boards)
}
