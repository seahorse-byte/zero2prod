use actix_web::{HttpResponse, Responder, post, web::Form};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
