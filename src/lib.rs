use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_check(_req: HttpRequest) -> impl Responder {
HttpResponse::Ok()
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

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
    App::new()
    .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8000")?
    .run();
    // No .await here!
    Ok(server)
    }