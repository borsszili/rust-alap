mod user;

use actix_web::{App, HttpServer, Responder};
use crate::user::user_class::{ test };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(test)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
