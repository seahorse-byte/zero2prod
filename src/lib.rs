use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .listen(listener)?
        .run();
    Ok(server)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
