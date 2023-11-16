use actix_web as web;
use maud::{
    html,
    Markup,
    DOCTYPE,
};

#[web::get("/")]
async fn index() -> web::Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        h1 { "Hello, World!" }
    })
}
