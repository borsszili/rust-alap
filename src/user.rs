pub mod user_class {
    use actix_web::{get, HttpResponse, Responder};

    #[get("/user")]
    pub async fn user_hello() -> impl Responder{
        HttpResponse::Ok().body("User.rs")
    }
}