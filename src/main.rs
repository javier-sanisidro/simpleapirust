use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn firstget(_req: HttpRequest) ->impl Responder{
    let first="Hello world";
    format!("{}",&first)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/hello_world",web::get().to(firstget))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}