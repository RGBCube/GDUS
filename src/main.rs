use axum::{
    routing,
    Router,
};
use maud::{
    html,
    Markup,
};

async fn index() -> Markup {
    html! {
        h1 { "Hello, World!" }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(index));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
