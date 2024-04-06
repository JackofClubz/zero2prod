use actix_web :: {web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet (req: HttpRequest) -> impl Responder{ // takes an http request and returns an implementation of the responder trait
    let name = req.match_info().get("name").unwrap_or("World"); // <- get. name indicates the url or endpoit the method is looking for, it's a routing function.
    format!("Hello {}!", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet)) // <- matches all request without anything after the slash "web::get()" is a shortcut for Route::new().guard(gaurd::Get())
            .route("/{name}", web::get().to(greet))
    }) // the guards (web::get()) specify conditions to check before passing to the handler 
    .bind("127.0.0.1:800")?
    .run()
    .await
}