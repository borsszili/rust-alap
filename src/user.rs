pub mod user_class {
    use actix_web::{get, HttpResponse, Responder, web};
    use actix_web::http::header::ContentType;

    pub fn get_services() -> actix_web::Scope {
        web::scope("/user")
            .service(test)
    }

    #[get("/list")]
    pub async fn test() -> impl Responder {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Hello world")
    }
}