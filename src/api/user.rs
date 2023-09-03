use serde::Serialize;

use serde::Deserialize;

use crate::app::App;
use crate::helper::input::InputContentVariants;
use crate::helper::stateful_list::StatefulList;
use crate::API_URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}

impl User {}

//TODO: Error handling
pub async fn sign_in(app: &mut App, credentials: Credentials) -> Option<User> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        None
    } else if credentials.email == "cookie" && credentials.password == "cookie" {
        let url = format!("{}/login/cookie/cookie", API_URL);

        let response = app
            .reqwest_client
            .get(&url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let user: User = serde_json::from_str(&response).unwrap();

        app.task_data = crate::api::task::get_all_tasks(app).await.unwrap();
        app.tasks.next();
        app.boards =
            StatefulList::with_items(crate::api::board::get_all_user_boards(app).await.unwrap());
        app.boards.next();

        return Some(user);
    } else {
        let client = &app.reqwest_client;
        let url = format!(
            "{}/login/{}/{}",
            API_URL, credentials.email, credentials.password
        );
        let response = client.get(&url).send().await.unwrap().text().await.unwrap();

        if response == "\"Logged in\"" {
            let url = format!("{}/login/cookie/cookie", API_URL);

            let response = client.get(&url).send().await.unwrap().text().await.unwrap();

            let user: User = serde_json::from_str(&response).unwrap();

            app.task_data = crate::api::task::get_all_tasks(app).await.unwrap();
            app.tasks.next();
            app.boards = StatefulList::with_items(
                crate::api::board::get_all_user_boards(app).await.unwrap(),
            );
            app.boards.next();

            Some(user)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub async fn sign_up(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let (email, password, name) = match &app.input_content.variant {
        InputContentVariants::SignUp {
            email,
            password,
            name,
        } => (email, password, name),
        _ => {
            return Err("Invalid input content variant".into());
        }
    };

    let client = &app.reqwest_client;
    let url = format!("{}/post/create_user", API_URL);
    let body = format!(
        "{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}",
        email, password, name
    );
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    app.credentials = Some(Credentials {
        email: "cookie".to_string(),
        password: "cookie".to_string(),
    });

    let user_acc = sign_in(app, app.credentials.clone().unwrap()).await;

    if user_acc.is_none() {
        return Err("Failed to sign in".into());
    } else {
        app.user = user_acc;
    }

    Ok(())
}
