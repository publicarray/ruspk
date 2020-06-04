#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate log;
use env_logger::Env;
use lazy_static::lazy_static;
#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use std::sync::{Arc, Mutex};
pub mod utils;

pub mod models;
pub mod routes;
pub mod synopackagelist;

#[cfg(feature = "sqlite")]
#[path = "schemas/sqlite/schema.rs"]
pub mod schema;
#[cfg(feature = "mysql")]
#[path = "schemas/mysql/schema.rs"]
pub mod schema;
#[cfg(feature = "postgres")]
#[path = "schemas/postgres/schema.rs"]
pub mod schema;

#[cfg(feature = "sqlite")]
type Connection = diesel::sqlite::SqliteConnection;
#[cfg(feature = "mysql")]
type Connection = diesel::mysql::MysqlConnection;
#[cfg(feature = "postgres")]
type Connection = diesel::pg::PgConnection;

#[cfg(feature = "sqlite")]
type DbId = i64;
#[cfg(feature = "mysql")]
type DbId = u64;
#[cfg(feature = "postgres")]
type DbId = i32;

#[cfg(feature = "sqlite")]
type Db64 = i64;
#[cfg(feature = "mysql")]
type Db64 = u64;
#[cfg(feature = "postgres")]
type Db64 = i64;

#[cfg(feature = "sqlite")]
type Db32 = i32;
#[cfg(feature = "mysql")]
type Db32 = u32;
#[cfg(feature = "postgres")]
type Db32 = i32;

#[cfg(feature = "sqlite")]
type Db8 = i8;
#[cfg(feature = "mysql")]
type Db8 = u8;
#[cfg(feature = "postgres")]
type Db8 = i8;

type DbPool = r2d2::Pool<ConnectionManager<Connection>>;
type DbConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref URL: String = std::env::var("URL").unwrap_or_else(|_| "https://packages.synocommunity.com".to_string());
}

pub struct AppData {
    pool: DbPool,
    cache_r: evmap::ReadHandle<String, String>,
    cache_w: Arc<Mutex<evmap::WriteHandle<String, String>>>,
    keyring: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let listen_addr = std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1".to_string());
    let listen_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let manager = ConnectionManager::<Connection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");
    let bind = format!("{}:{}", listen_addr, listen_port);
    info!("Starting server at: {}", &bind);

    // get public key / keychain
    let public_key = match std::env::var("PUBLIC_KEY_FILE") {
        Ok(public_key_filename) => {
            debug!("loading public key: {}", public_key_filename);
            match utils::read_file(&public_key_filename) {
                Ok(public_key) => public_key,
                Err(err) => {
                    error!("Unable to get public key '{}'. {}", public_key_filename, err);
                    "".to_string()
                }
            }
        }
        Err(err) => {
            warn!("PUBLIC_KEY_FILE {}", err);
            "".to_string()
        }
    };

    let (cache_r, raw_cache_w) = evmap::new();
    let cache_w = Arc::new(Mutex::new(raw_cache_w));
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(AppData {
                pool: pool.clone(),
                cache_r: cache_r.clone(),
                cache_w: cache_w.clone(),
                keyring: public_key.clone(),
            })
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello").route(web::get().to(routes::index)))
            .service(web::resource("/hello/{name}").route(web::get().to(routes::index)))
            .service(web::resource("/").route(web::get().to(routes::syno)))
            .service(web::resource("/").route(web::post().to(routes::syno)))
            .service(web::resource("/package").route(web::get().to(routes::list_packages)))
            .service(web::resource("/package/{id}").route(web::get().to(routes::get_package_version)))
    })
    .bind(&bind)?
    .run()
    .await
}
