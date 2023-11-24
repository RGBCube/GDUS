use actix_web as web;
use actix_web::web::{
    Data,
    Query,
};
use maud::{
    html,
    Markup,
    DOCTYPE,
};
use sqlx::SqlitePool;

#[web::get("/")]
async fn index(data: Data<SqlitePool>) -> web::Result<Markup> {
    sqlx::query_as::<(String, String)>(
        r"
       TODO
    ",
    );

    Ok(html! {
        (DOCTYPE)
        h1 { "Hello, World!" }
    })
}
