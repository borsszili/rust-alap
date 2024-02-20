pub mod user_class {
    use actix_web::{get, HttpResponse, Responder, Scope, web};
    use actix_web::http::header::ContentType;

    #[get("/user/list")]
    pub async fn test() -> impl Responder {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("Hello world")
    }
}