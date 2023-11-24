mod index;
mod submit;

use std::io;

use actix_web as web;
use sqlx::{
    sqlite::SqliteConnectOptions,
    SqlitePool,
};

#[web::main]
async fn main() -> io::Result<()> {
    let db = SqlitePool::connect_with(SqliteConnectOptions::new().filename("data.db"))
        .await
        .unwrap();

    sqlx::query(
        r"
        CREATE TABLE IF NOT EXISTS reminders (
            date    TEXT,
            message TEXT
        )
    ",
    )
    .execute(&db)
    .await
    .unwrap();

    web::HttpServer::new(move || {
        web::App::new()
            .app_data(web::web::Data::new(db.clone()))
            .service(index::index)
            .service(submit::submit)
            .service(submit::submit_form)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
