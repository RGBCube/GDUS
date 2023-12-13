use actix_web as web;
use actix_web::web::{
    Data,
    Query,
};
use chrono::NaiveDateTime;
use maud::{
    html,
    Markup,
    PreEscaped,
    DOCTYPE,
};
use sqlx::SqlitePool;
#[derive(Debug, serde::Deserialize)]
pub struct Reminder {
    date: String,
    message: String,
}

#[web::get("/submit-form")]
async fn submit_form(
    data: Data<SqlitePool>,
    Query(reminder): Query<Reminder>,
) -> web::Result<Markup> {
    println!("{reminder:?}");

    sqlx::query(
        r"
        INSERT INTO
            reminders (date, message)
        VALUES
            (?, ?)
    ",
    )
    .bind(
        NaiveDateTime::parse_from_str(&reminder.date, "%Y-%m-%dT%H:%M")
            .unwrap()
            .timestamp() as i64,
    )
    .bind(reminder.message)
    .execute(&**data)
    .await
    .expect("Failed to save reminder.");

    Ok(html! {
        (DOCTYPE)
        h1 { "Kaydedildi." }
        p { "Ana sayfaya geri yönlendiriliyorsun..." }
        script type="text/javascript" {(PreEscaped(r#"
            setTimeout(function() { window.location.href = "/"; }, 5000);
        "#))}
    })
}

#[web::get("/submit")]
async fn submit() -> web::Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        form action="/submit-form" {
            button type="submit" { "Kaydet" }
            br;
            label for="date" { "Tarih:" }
            input type="datetime-local" id="date" name="date";
            br;
            label for="message" { "Mesaj:" }
            input id="message" name="message";
        }
    })
}
