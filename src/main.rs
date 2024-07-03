use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use zero2prod::run;

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
// Bubble up the io::Error if we failed to bind the address
// Otherwise call .await on our Server
run()?.await
}
