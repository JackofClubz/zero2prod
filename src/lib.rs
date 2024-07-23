use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder,};
use actix_web::dev::Server;
use serde::Deserialize;

#[derive(serde::Deserialize)]
struct FormData{
    email:String,
    name:String
}

async fn health_check() -> HttpResponse {
HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse{
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
    App::new()
    .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

pub fn run(listener:TcpListener) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(||{
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    Ok(server)
}
    