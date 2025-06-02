use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, post, web::Form};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(subscribe))
        .listen(listener)?
        .run();
    Ok(server)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}
