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

            h1 {
                text-align: center;
                color: #333;
            }

            .links {
                text-align: center;
                margin-top: 30px;
            }

            .links a {
                display: inline-block;
                margin: 10px;
                padding: 8px 16px;
                text-decoration: none;
                color: #fff;
                background-color: #4caf50;
                border-radius: 4px;
                transition: background-color 0.3s ease;
            }

            .links a:hover {
                background-color: #45a049;
            }
        "#}

        div class="links" {
            a href="/submit" { "Hatırlatıcı Koy" }
            a href="/view" { "Hatırlatıcıları Görüntüle" }
        }
    })
}
