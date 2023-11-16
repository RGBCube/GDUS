use actix_web as web;
use actix_web::web::Query;
use maud::{
    html,
    Markup,
    DOCTYPE,
};

#[derive(Debug, serde::Deserialize)]
pub struct Reminder {
    date: String,
    message: String,
}

#[web::get("/submit-form")]
async fn submit_form(Query(reminder): Query<Reminder>) -> web::Result<Markup> {
    println!("{reminder:?}");

    Ok(html! {
        (DOCTYPE)
        h1 { "Kaydedildi." }
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
