use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
