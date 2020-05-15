#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;


use actix_web::{middleware, web, App, HttpServer};

// pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;

// use diesel::PgConnection;
use diesel::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};


type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type DbConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::MysqlConnection>>;

const URL: &str = "http://packages.synocommunity.com";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello/").route(web::get().to(routes::index)))
            .service(web::resource("/hello/{name}").route(web::get().to(routes::index)))
            .service(web::resource("/").route(web::get().to(routes::syno)))
            .service(web::resource("/").route(web::post().to(routes::syno_post)))
            .service(web::resource("/package").route(web::get().to(routes::list_packages)))
    })
    .bind(&bind)?
    .run()
    .await
}
