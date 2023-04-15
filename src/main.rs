use axum::{Router};
use axum::routing::get;

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

async fn hello_world() -> String {
    String::from("Hello, world!")
}

async fn give_work() {
    let new_work = Work{
        id: 1,
        work_code: "jpmc".to_string(),
        add_up_to: 100,
        done: false,
    };

    let work_json = serde_json::to_string(&new_work).unwrap();
    println!("{work_json}");
}