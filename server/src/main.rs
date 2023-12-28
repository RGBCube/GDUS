mod index;
mod submit;
mod view;

use std::io;

use actix_cors::Cors;
use actix_web as web;
use sqlx::{
    sqlite::SqliteConnectOptions,
    SqlitePool,
};

#[web::main]
async fn main() -> io::Result<()> {
    let db = SqlitePool::connect_with(
        SqliteConnectOptions::new()
            .filename("data.db")
            .create_if_missing(true),
    )
    .await
    .expect("Failed to connect to SQLite database.");

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS reminders (
            date    DATETIME NOT NULL,
            message TEXT     NOT NULL,
            led     TINYINT  NOT NULL
        )
    ",
    )
    .execute(&db)
    .await
    .expect("Failed to create table reminders.");

    let data = web::web::Data::new(db);

    web::HttpServer::new(move || {
        web::App::new()
            .app_data(data.clone())
            .wrap(Cors::permissive())
            .service(index::index)
            .service(submit::submit)
            .service(submit::submit_form)
            .service(view::view)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
