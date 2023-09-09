use std::fs;

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
pub async fn sign_in(
    app: &mut App,
    credentials: Credentials,
    cookie: Option<String>,
) -> Option<User> {
    if credentials.email.is_empty() || credentials.password.is_empty() {
        None
    } else if credentials.email == "cookie" && credentials.password == "cookie" {
        let url = format!("{}/login/cookie/cookie?device_identifier=tty", API_URL);

        let cookie_formated = if cookie.clone().is_some() {
            println!("cookie: {}", cookie.clone().unwrap());
            format!("user_uuid={}", cookie.unwrap())
        } else {
            println!("cookie: None");
            "".to_string()
        };

        let response: String;

        if cookie_formated.is_empty() {
            response = app
                .reqwest_client
                .get(&url)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        } else {
            response = app
                .reqwest_client
                .get(&url)
                .header("Cookie", cookie_formated)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        };

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
            "{}/login/{}/{}?device_identifier=tty",
            API_URL, credentials.email, credentials.password
        );
        let response = client.get(&url).send().await.unwrap().text().await.unwrap();

        if response == "\"Logged in\"" {
            let url = format!("{}/login/cookie/cookie?device_identifier=tty", API_URL);

            let response = client.get(&url).send().await.unwrap().text().await.unwrap();

            let user: User = serde_json::from_str(&response).unwrap();

            app.task_data = crate::api::task::get_all_tasks(app).await.unwrap();
            app.tasks.next();
            app.boards = StatefulList::with_items(
                crate::api::board::get_all_user_boards(app).await.unwrap(),
            );
            app.boards.next();

            // save the cookie to ~/.config/quicktick-tty/cookie.txt
            let cookie_dir = dirs::config_dir().unwrap().join("quicktick-tty");
            let cookie_file = cookie_dir.join("cookie.txt");
            fs::create_dir_all(cookie_dir).unwrap();
            fs::write(cookie_file, user.uuid.clone()).unwrap();

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
    let url = format!("{}/post/create_user?device_identifier=tty", API_URL);
    let body = format!(
        "{{\"email\":\"{}\",\"password\":\"{}\",\"name\":\"{}\"}}",
        email, password, name
    );
    let _response = client
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

    let user_acc = sign_in(app, app.credentials.clone().unwrap(), None).await;

    if user_acc.is_none() {
        return Err("Failed to sign in".into());
    } else {
        app.user = user_acc;
    }

    Ok(())
}

pub async fn log_out(app: &mut App) {
    let cookie_dir = dirs::config_dir().unwrap().join("quicktick-tty");
    let cookie_file = cookie_dir.join("cookie.txt");
    fs::remove_file(cookie_file).unwrap();

    app.reqwest_client
        .get(&format!("{}/logout?device_identifier=tty", API_URL))
        .send()
        .await
        .unwrap();

    app.user = None;
    app.credentials = Some(Credentials {
        email: "".to_string(),
        password: "".to_string(),
    });
    app.boards = StatefulList::with_items(Vec::new());
    app.tasks = StatefulList::with_items(Vec::new());
    app.task_data = Vec::new();
    app.input_content.visible = false;
    app.input_content.selected_input = 0;
    app.selected_widget = true;
}
