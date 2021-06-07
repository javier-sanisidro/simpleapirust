use actix_web::{App, HttpRequest, HttpResponse,http::StatusCode, HttpServer, Responder, get, middleware::Logger, post, web::{self, Data}};
extern crate r2d2;
extern crate r2d2_mysql;
use serde::{Deserialize, Serialize};
use std::env;
use r2d2_mysql::mysql::{ OptsBuilder, QueryResult, from_row, prelude::FromRow};
use std::sync::Arc;
use r2d2::Pool;
use r2d2_mysql::MysqlConnectionManager;


fn get_pool() -> Option<Arc<Pool<MysqlConnectionManager>>> {
    let mut o = OptsBuilder::new();
    let db_name = match env::var("HOST_DATABASE") {
        Ok(val) => val,
        Err(_e) => "ozona".to_string(),
       };
    o.db_name(Option::Some(db_name));
    let name = match env::var("USER_DATABASE") {
        Ok(val) => val,
        Err(_e) => "root".to_string(),
       };
    o.user(Option::Some(name));
    let password = match env::var("PASSWORD_DATABASE") {
        Ok(val) => val,
        Err(_e) => "abc123.".to_string(),
       };
    o.pass(Option::Some(password));
    let database_url = match env::var("URL_DATABASE") {
        Ok(val) => val,
        Err(_e) => "localhost".to_string(),
       };
    o.ip_or_hostname(Option::Some(database_url));
    
    
 
    let manager = r2d2_mysql::MysqlConnectionManager::new(o);
 
    println!("Getting pool");
 
  let pool = Arc::new(r2d2::Pool::new(manager).unwrap());
  

    return Option::Some(pool);
}

struct AppState {
    app_name: String,
    pool: Arc<Pool<MysqlConnectionManager>>,
}
#[derive(Deserialize, Serialize)]
struct Personas{
    person_id: i32,
    person_name: String,
}
async fn index() ->  HttpResponse {
    HttpResponse::Ok().body("Hello world")
}


async fn firstget(req: HttpRequest) ->HttpResponse{
    let name = req.match_info().get("name").unwrap_or("World");
    let mut body=String::from("Hello ");
    body=body+&name;
    HttpResponse::Ok().body(body)
}





async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("esto es una prueba")
}





#[post("/ingresar_personas")]
async fn hello2(info: web::Json<Personas>, data: web::Data<AppState>) -> HttpResponse {
    println!("Acces to post person ");
   
  
    let pool = &data.pool;
 
  
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
    println!("Connect to db");
    let ingreso= Personas{
        person_id: info.person_id,
        person_name: info.person_name.clone(),
    };
    println!("get data from json");
    let command=String::from("INSERT INTO person VALUES(?,?)");
    match conn.prep_exec(command, (ingreso.person_id,ingreso.person_name )){
        Ok(_val) => println!("Correct Added"),
        Err(_e) => println!("Failed  Added"),
       };
    HttpResponse::Ok().body("Correct added")
}




#[get("/persons/{id}")]
async fn persons(info: web::Path<i32>, data: web::Data<AppState>) ->HttpResponse {
    println!("Acces to get person by id ");
    
 
    let pool = &data.pool;
 
 
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
    println!("Connect to db");
    
    let param = info.into_inner();
    println!("search into db the person with the parameter");
    let qr: QueryResult = conn.prep_exec("select person_id, person_name from person where person_id = ?", (param, )).unwrap();
 
    let mut rec: Option<(i32, String)> = None;
    println!("Convert data");
    for row in qr {
        rec = Some(from_row(row.unwrap()));
        break;
    }
    if rec==None{
        HttpResponse::BadRequest().body("No hay autor")
    }else{
    let unwrap_rec = rec.unwrap();
    println!("Hello {} ! \n",  unwrap_rec.1);
    HttpResponse::Ok().body(unwrap_rec.1)
}
}




#[get("/persona")]
async fn persons2( data: web::Data<AppState>) -> impl Responder {
   
    println!("Acces to get all person");
 
    let pool = &data.pool;
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
    println!("Connect to db");
    println!("take all person in a vector");
    let all_persons: Vec<Personas> =
    conn.prep_exec("SELECT person_id, person_name from person", ())

        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (person_id, person_name) = {
                    let row = row;
                    FromRow::from_row(row)
                };

                Personas {
                    person_id,
                    person_name,
                }
            }).collect()
        }).unwrap(); // Unwrap `Vec<Person>`
        println!("convert all data to vec<struct>");
        let mut listado:Vec<Personas>=Vec::new();
        for items in all_persons.iter(){
           
            let estructura=Personas{
                person_id: items.person_id,
                person_name: items.person_name.clone(),
            };
            listado.push(estructura);
        }
        println!("put in web vec<struc>");
        HttpResponse::Ok().json(listado)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        app_name: String::from("ozona"),
        pool: get_pool().unwrap(),
    });
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    


    create_table(app_data.clone());
   
    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(Logger::new("%a %{User-Agent}i"))
        .wrap(Logger::new("%a %{FOO}e"))
        .app_data(app_data.clone()
        ).service(persons)
        .service(persons2)
        .service(hello2)
            .route("/patata",web::get().to(hello))
            .route("/", web::get().to(firstget))
            .route("/{name}", web::get().to(firstget))

            
    })
    // Cambie 127.0.0.1 por 0.0.0.0 dentro de los docker intentemos no referirnos a localhost y el puerto donde se va a ejecutar la aplicacion al 80 
    // podria dejarlo que se ejecute en el puerto 8080 y a la hora de ejecutarlo con docker utilizar -p 80:8080 -p <PUERTO-HOST>:<PUERTO-CONTENEDOR>
    .bind(("0.0.0.0", 80))?
    .run()
    .await
    
}
fn create_table(data: Data<AppState>){
    let pool = &data.pool;
    let pool = pool.clone();
    let mut conn = pool.get().unwrap();
    println!("Create table");
    let command= String::from(" CREATE TABLE IF NOT EXISTS  person( person_id int auto_increment,person_name varchar(100) null,constraint person_pk primary key (person_id))");
    match conn.prep_exec(command, ()){
        Ok(_val) => println!("Create table ok"),
        Err(_e) =>  println!("Create table not ok"),
       };
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};

    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = index().await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
    #[actix_rt::test]
    async fn test_index_ok() {
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = hello(req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_index_not_ok() {
        let req = test::TestRequest::default().to_http_request();
        let resp = hello().await;
        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }
}