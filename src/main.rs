use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {

    let app = Router::new().route(
        "/hello",
        get(hello_world)
    );

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()

}

async fn hello_world() -> String {
    String::from("Hello, world!")
}
