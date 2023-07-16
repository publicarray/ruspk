#[macro_use]
extern crate log;
use actix_web::web::Data;
use anyhow::Context;
use env_logger::Env;
use lazy_static::lazy_static;
#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate chrono;

extern crate regex;

extern crate sequoia_openpgp as openpgp;
use openpgp::Cert;
use openpgp::{parse::Parse, serialize::SerializeInto};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys, ec_private_keys};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
mod claims;

use diesel::r2d2::{self, ConnectionManager};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use evmap_derive::ShallowCopy;

use crate::api::*;

pub mod utils;
pub mod filestorage;

pub mod api;
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
type DbId = u64; //Unsigned<Bigint>
#[cfg(feature = "postgres")]
type DbId = i32; //Int4

#[cfg(feature = "sqlite")]
type Db64 = i64;
#[cfg(feature = "mysql")]
type Db64 = u64;
#[cfg(feature = "postgres")]
type Db64 = i64; //BigInt

//mysql(Integer),postgres(Int4),sqlite(Integer) = i32
#[cfg(feature = "sqlite")]
type Dbu32 = i32;
#[cfg(feature = "mysql")]
type Dbu32 = u32;
#[cfg(feature = "postgres")]
type Dbu32 = i32; //Int4

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

    #[derive(Copy, Clone, Debug)]
    pub static ref CACHE_TTL: String = std::env::var("CACHE_TTL").unwrap_or_else(|_| "600".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_TYPE: String = std::env::var("STORAGE_TYPE").unwrap_or_else(|_| "filesystem".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_PATH: String = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "packages".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_S3_API: String = std::env::var("STORAGE_S3_API").unwrap_or_else(|_| "https://s3.amazonaws.com".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_S3_ID: String = std::env::var("STORAGE_S3_ID").unwrap_or_else(|_| "".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_S3_SECRET_KEY: String = std::env::var("STORAGE_S3_SECRET_KEY").unwrap_or_else(|_| "".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_S3_REGION: String = std::env::var("STORAGE_S3_REGION").unwrap_or_else(|_| "auto".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref STORAGE_S3_BUCKET: String = std::env::var("STORAGE_S3_BUCKET").unwrap_or_else(|_| "".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref PGP_KEY_PATH: String = std::env::var("PGP_KEY_PATH").unwrap_or_else(|_| "pgpkey.pem".to_string());

    #[derive(Copy, Clone, Debug)]
    pub static ref GNUPG_TIMESTAMP_URL: String = std::env::var("GNUPG_TIMESTAMP_URL").unwrap_or_else(|_| "http://timestamp.synology.com/timestamp.php".to_string());
}

// Cache Type
#[derive(Clone, Debug, Hash, PartialEq, ShallowCopy)]
pub struct CacheValue {
    http_response: Arc<String>,
    insert_time: Arc<Instant>,
}
impl Eq for CacheValue {}

pub struct AppData {
    pool: DbPool,
    cache_r: evmap::ReadHandle<String, CacheValue>,
    cache_w: Arc<Mutex<evmap::WriteHandle<String, CacheValue>>>,
    keyring: Option<String>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    lazy_static::initialize(&CACHE_TTL);
    lazy_static::initialize(&URL);
    lazy_static::initialize(&PGP_KEY_PATH);
    lazy_static::initialize(&GNUPG_TIMESTAMP_URL);
    trace!("CACHE_TTL:{}", *CACHE_TTL);
    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let listen_addr = std::env::var("LISTEN").unwrap_or_else(|_| "127.0.0.1".to_string());
    let listen_port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let tls_key = std::env::var("TLS_KEY").unwrap_or_else(|_| "server/key.pem".to_string());
    let tls_cert = std::env::var("TLS_CERT").unwrap_or_else(|_| "server/cert.pem".to_string());
    let manager = ConnectionManager::<Connection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");
    let mut tls_config:Option<rustls::ServerConfig> = None;
    if std::path::Path::new(&tls_key).exists() {
        tls_config = Some(load_rustls_config(&tls_key, &tls_cert));
    }

    let bind = format!("{}:{}", listen_addr, listen_port);
    info!("Starting server at: {}", &bind);

    // OpenPGP
    //let mut tsk:Option<Cert> = None;
    let mut pgp_key:Option<String> = None;
    // if file at &PGP_KEY_PATH exists
    if std::path::Path::new(&*PGP_KEY_PATH).exists() {
        let temp_tsk = Cert::from_file(&*PGP_KEY_PATH)
            .context("Failed to read signing key")
            .unwrap();
        let temp_key = String::from_utf8(temp_tsk.armored().to_vec().unwrap()).unwrap();
        debug!("Key: {}", temp_key);
        info!("Loaded Key: {}", temp_tsk.fingerprint());
        //tsk = Some(temp_tsk);
        pgp_key = Some(temp_key);
    } else {
        warn!("No signing key file found at '{}'. SPK Signing is disabled!", &*PGP_KEY_PATH);
    }

    let (cache_r, raw_cache_w) = evmap::new();
    let cache_w = Arc::new(Mutex::new(raw_cache_w));
    // Start HTTP server
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("https://localhost:3000")
            .allow_any_origin()
            .allow_any_header()
            .send_wildcard()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .max_age(3600);
        let auth = HttpAuthentication::bearer(auth::validator);
        App::new()
            .wrap(middleware::NormalizePath::trim())
            //.wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Always))
            // .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(cors)
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(AppData {
                pool: pool.clone(),
                cache_r: cache_r.clone(),
                cache_w: cache_w.clone(),
                keyring: pgp_key.clone(),
            }))
            .wrap(middleware::Logger::default())
            //.service(web::resource("/hello").route(web::get().to(routes::index)))
            //.service(web::resource("/hello/{name}").route(web::get().to(routes::index)))
            .service(web::resource("/nas").route(web::get().to(routes::syno)))
            .service(web::resource("/nas").route(web::post().to(routes::syno)))
            //.service(web::resource("/package").route(web::get().to(routes::list_packages)))
            //.service(web::resource("/package/{id}").route(web::get().to(routes::get_package_version)))
            .service(auth::login)
            .service(auth::new_reset)
            // .service(auth::reset)
            // home /packages
            .service(package::get_all)
            // home /package detail view
            .service(package::get)
            .service(version::get_all)
            .service(build::get_all)
            // spksrc POST new build/package api_key endpoint
            .service(build::post)
            // admin api
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(auth::profile)
                    .service(user::get_all)
                    .service(user::delete)
                    .service(build::delete)
                    .service(build::active)
                    .service(build::delete_id)
                    .service(architecture::post)
                    .service(architecture::delete)
                    .service(architecture::get_all)
                    .service(firmware::post)
                    .service(firmware::get_all)
                    .service(version::delete)
                    .service(version::delete_id)
                    .service(screenshot::get_all)
                    // .service(screenshot::post)
                    .service(screenshot::delete)
                    .service(screenshot::delete_id)
                    .service(package::post)
                    .service(package::delete)
                    .service(package::delete_id),
            )
            .service(fs::Files::new("/", &*STORAGE_PATH))
        // .service(
        //     web::scope("/admin").service(
        //         fs::Files::new("/", "frontend/dist/admin")
        //             .index_file("index.html")
        //             .prefer_utf8(true),
        //     ),
        // )
        // .service(
        //     fs::Files::new("/", "frontend/dist")
        //         .index_file("index.html")
        //         .prefer_utf8(true),
        // )
    });

    if let Some(tls_config) = tls_config {
        info!("TLS=Enabled");
        server.bind_rustls(&bind, tls_config)?
            .run()
            .await
    } else {
        server.bind(&bind)?
            .run()
            .await
    }
}

use std::fs::File;
use std::io::BufReader;
fn load_rustls_config(key: &String, cert: &String) -> rustls::ServerConfig {
    // init server config builder with safe defaults
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open(cert).unwrap());
    let key_file = &mut BufReader::new(File::open(key).unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = ec_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        keys = pkcs8_private_keys(key_file)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();  
    }
    if keys.is_empty() {
        keys = rsa_private_keys(key_file)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();  
    }
    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("TLS: Could not locate private keys. {}", key);
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}