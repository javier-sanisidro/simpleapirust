use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web::{self, Data}};
extern crate r2d2;
extern crate r2d2_mysql;
use r2d2_mysql::mysql::{OptsBuilder, QueryResult, from_row};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;

fn get_pool() -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let mut o = OptsBuilder::new();
    o.db_name(Option::Some("ozona"));
    o.user(Option::Some("root"));
    o.pass(Option::Some("abc123."));
 
    let manager = r2d2_mysql::MysqlConnectionManager::new(o);
 
    println!("Getting pool");
 
    let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
    return Option::Some(pool);
}
struct AppState {
    app_name: String,
    pool: Arc<Pool<MysqlConnectionManager>>,
}
struct Personas{
    pub id: u32,
    pub name: String,
}

async fn firstget(req: HttpRequest) ->impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
#[get("/patata")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("esto es una prueba")
}
#[get("/persons/{id}")]
async fn index(info: web::Path<i32>, data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
 
    let pool = &data.pool;
 
 
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
 
    let param = info.into_inner();
    let qr: QueryResult = conn.prep_exec("select person_id, person_name from person where person_id = ?", (param, )).unwrap();
 
    let mut rec: Option<(i32, String)> = None;
 
    for row in qr {
        rec = Some(from_row(row.unwrap()));
        break;
    }
 
    let unwrap_rec = rec.unwrap();
    format!("Hello {} ({})! \n from {}",  unwrap_rec.1, unwrap_rec.0, app_name)
}
#[get("/persona")]
async fn index2( data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name; // <- get app_name
 
    let pool = &data.pool;
 
 
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
    let qr=conn.query("select person_id, person_name from person").unwrap();
    //  let qr: QueryResult = conn.prep_exec("select person_id, person_name from person where person_id = ?", (param, )).unwrap();
    let mut listado: Vec<Personas>=Vec::new();
    for row in qr {
        let identificador= row.get(0);
        let nombre= row.get(1);
        let patata =Personas{
            id: identificador,
            name: nombre,
        };
        listado.push(patata);
    }

    HttpResponse::Ok().json(listado)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        app_name: String::from("ozona"),
        pool: get_pool().unwrap(),
    });
    HttpServer::new(move || {
        App::new()
        .app_data(app_data.clone()
        ).service(index)
        .service(hello)
            .route("/", web::get().to(firstget))
            .route("/{name}", web::get().to(firstget))
    })
    // Cambie 127.0.0.1 por 0.0.0.0 dentro de los docker intentemos no referirnos a localhost y el puerto donde se va a ejecutar la aplicacion al 80 
    // podria dejarlo que se ejecute en el puerto 8080 y a la hora de ejecutarlo con docker utilizar -p 80:8080 -p <PUERTO-HOST>:<PUERTO-CONTENEDOR>
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}