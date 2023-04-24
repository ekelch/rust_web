use std::fmt::Formatter;
use axum::{Json, Router};
use axum::routing::get;
use libsql_client::{new_client_from_config, Config, DatabaseClient};
use libsql_client::hrana::Client;
use serde::{Serialize, Deserialize};


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/hello",get(hello_world))
        .route("/", get(give_work));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()

}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct Work {
    pub id: i32,
    pub work_code: String,
    pub add_up_to: i32,
    pub done: bool,
}

impl std::fmt::Display for Work {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, work code: {}, active: {}",
            self.id, self.work_code, !self.done
        )
    }
}

// async fn get_client() -> DatabaseClient {
//     client = new_client_from_config(Config {
//         url: "libsql://your-database.turso.io".try_into()?,
//         auth_token: Some(String::from("your-auth-token")),
//     }).await?;
// }

async fn hello_world() -> String {
    String::from("Hello, world!")
}

async fn give_work() -> Json<Work> {
    let new_work = Work{
        id: 1,
        work_code: "jpmc".to_string(),
        add_up_to: 100,
        done: false,
    };
    Json(new_work)
}