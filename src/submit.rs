use actix_web as web;
use actix_web::web::{
    Data,
    Query,
};
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
    .bind(reminder.date)
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
            ul {
                li class="li-button" {
                    button type="submit" { "Kaydet" }
                }
                li {
                    label for="date" { "Tarih:" }
                    input type="datetime-local" id="date" name="date";
                }
                li {
                    label for="message" { "Mesaj:" }
                    input id="message" name="message";
                }
            }
        }
    })
}
