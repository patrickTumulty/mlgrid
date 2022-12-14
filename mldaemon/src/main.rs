use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn ping(req: HttpRequest) -> impl Responder {
    format!("I'm Here!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/ping", web::get().to(ping))
    }).bind(("127.0.0.1", 8080))?
      .run()
      .await
}






