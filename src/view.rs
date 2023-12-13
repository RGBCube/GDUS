use actix_web as web;
use actix_web::web::{
    Data,
    Query,
};
use chrono::Local;
use maud::{
    html,
    Markup,
    DOCTYPE,
};
use sqlx::SqlitePool;

#[web::get("/view")]
async fn view(data: Data<SqlitePool>) -> web::Result<Markup> {
    let reminders = sqlx::query_as::<_, (i64, String)>(
        r"
        SELECT
          *
        FROM
          reminders
        WHERE
          date > ?
    ",
    )
    .bind(Local::now().timestamp() as i64)
    .fetch_all(&**data)
    .await
    .expect("Failed to fetch reminder.");
    println!("{reminders:?}");

    Ok(html! {
        (DOCTYPE)
        ul {
            @for reminder in reminders {
                li {
                    h3 { (reminder.0) }
                    p { (reminder.1) }
                }
            }
        }
    })
}
