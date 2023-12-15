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

    let date_time = NaiveDateTime::parse_from_str(&reminder.date, "%Y-%m-%dT%H:%M").unwrap();

    sqlx::query(
        r"
        INSERT INTO
            reminders (date, message)
        VALUES
            (?, ?)
    ",
    )
    .bind(date_time)
    .bind(reminder.message)
    .execute(&**data)
    .await
    .expect("Failed to save reminder.");

    Ok(html! {
        (DOCTYPE)

        style {r#"
            body {
                font-family: sans;
                background-color: #f4f4f4;
                margin: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                flex-direction: column;
            }

            h1 {
                color: #333;
                margin-bottom: 20px;
            }

            p {
                color: #666;
                margin-bottom: 40px;
            }
        "#}

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

        style {r#"
            body {
                font-family: sans;
                background-color: #f4f4f4;
                margin: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
            }

            form {
                background-color: #fff;
                padding: 20px;
                border-radius: 8px;
                box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
            }

            label {
                display: block;
                margin-bottom: 8px;
            }

            input {
                width: 100%;
                padding: 8px;
                margin-bottom: 16px;
                box-sizing: border-box;
                border: 1px solid #ccc;
                border-radius: 4px;
            }

            button {
                background-color: #4caf50;
                color: #fff;
                padding: 10px 20px;
                border: none;
                border-radius: 4px;
                cursor: pointer;
            }

            button:hover {
                background-color: #45a049;
            }
        "#}

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
