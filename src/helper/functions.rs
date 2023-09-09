use std::fs;

use crate::{
    api::{
        self,
        user::{sign_in, Credentials},
    },
    app::App,
    helper::stateful_list::StatefulList,
};

pub async fn refetch_data(mut app: &mut App) {
    let config_dir = dirs::config_dir().unwrap().join("quicktick-tty");
    let cookie_file = config_dir.join("cookie.txt");

    if cookie_file.exists() {
        let uuid = fs::read_to_string(cookie_file).unwrap();
        let res = sign_in(
            app,
            Credentials {
                email: "cookie".to_string(),
                password: "cookie".to_string(),
            },
            Some(uuid),
        )
        .await;

        if let Some(user) = res {
            app.user = Some(user);
            app.task_data = crate::api::task::get_all_tasks(app).await.unwrap();
            app.tasks.next();
            app.boards =
                StatefulList::with_items(api::board::get_all_user_boards(app).await.unwrap());
            app.boards.next();
        }
    }
}
