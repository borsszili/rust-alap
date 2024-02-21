mod user;

use actix_web::{App, HttpServer};
use crate::user::user_class::{get_services};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_services())
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
