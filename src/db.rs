extern crate r2d2;
extern crate r2d2_mysql;
 
use actix_web::{get, web, App, HttpServer, Responder};
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