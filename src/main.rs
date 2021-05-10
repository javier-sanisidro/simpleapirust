use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn firstget(_req: HttpRequest) ->impl Responder{
    let first="Hello world";
    format!("{}",&first)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/",web::get().to(firstget))
    })
    // Cambie 127.0.0.1 por 0.0.0.0 dentro de los docker intentemos no referirnos a localhost y el puerto donde se va a ejecutar la aplicacion al 80 
    // podria dejarlo que se ejecute en el puerto 8080 y a la hora de ejecutarlo con docker utilizar -p 80:8080 -p <PUERTO-HOST>:<PUERTO-CONTENEDOR>
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}