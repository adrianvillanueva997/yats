use axum::{routing::get, Router};
use tracing::{info, instrument};
#[tokio::main]
#[instrument]
async fn main() {
    tracing_subscriber::fmt::init();
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    info!("Server up and running!");

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
