use crate::{
    api::user::{sign_in, Credentials},
    app::App,
};

pub async fn refetch_data(app: &mut App) {
    app.credentials = Some(Credentials {
        email: "cookie".to_string(),
        password: "cookie".to_string(),
    });

    sign_in(app, app.credentials.clone().unwrap()).await;
}
