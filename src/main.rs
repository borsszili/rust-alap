mod user;

use actix_web::{App, HttpServer, Responder};
use crate::user::user_class;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_class::user_hello)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
