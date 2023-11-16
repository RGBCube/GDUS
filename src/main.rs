mod index;
mod submit;

use std::io;

use actix_web as web;

#[web::main]
async fn main() -> io::Result<()> {
    web::HttpServer::new(|| {
        web::App::new()
            .service(index::index)
            .service(submit::submit)
            .service(submit::submit_form)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
