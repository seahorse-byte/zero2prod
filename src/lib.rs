use actix_web::{App, HttpResponse, HttpServer, Responder, get};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
