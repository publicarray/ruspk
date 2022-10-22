#[macro_use]
extern crate log;
use actix_web::web::Data;
use anyhow::{Context};
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
use openpgp::{
    parse::{Parse},
    serialize::SerializeInto,
};

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
    keyring: String,
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
    let manager = ConnectionManager::<Connection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");
    let bind = format!("{}:{}", listen_addr, listen_port);
    info!("Starting server at: {}", &bind);

    // get public key / keychain
    // let pgp_key_path = std::env::var("PGP_KEY_PATH").unwrap_or_else(|_| "pgpkey.pem".to_string());
    // let tsk = openpgp::Cert::from_file(&pgp_key_path).context("Failed to read key").unwrap();
    let tsk = openpgp::Cert::from_file(&*PGP_KEY_PATH)
        .context("Failed to read key")
        .unwrap();

    // let mut keys = Vec::new();
    // let p = &crate::openpgp::policy::StandardPolicy::new();
    // let mut n = 0;
    // for key in tsk.keys().with_policy(p, None).alive().revoked(false).for_signing().secret().map(|ka| ka.key()) {
    //     keys.push({
    //         let mut key = key.clone();
    //         if key.secret().is_encrypted() {
    //             // let password = read_from_sdin (Some(&format!("Please enter password to decrypt {}/{}: ",tsk, key)))?;
    //             let password = "";
    //             let algo = key.pk_algo();
    //             key.secret_mut()
    //                 .decrypt_in_place(algo, &password.into())
    //                 .context("decryption failed").unwrap();
    //         }
    //         n += 1;
    //         key.into_keypair().unwrap();
    //     });
    // }

    // if n==0 {
    //     error!("No valid signing key found");
    // }

    // let keypair = tsk
    //     .keys().unencrypted_secret()
    //     .with_policy(p, None).supported().alive().revoked(false).for_signing()
    //     .next().unwrap().key().clone().into_keypair().unwrap();

    let public_key = String::from_utf8(tsk.armored().to_vec().unwrap()).unwrap();
    debug!("Public Key: {}", public_key);
    info!("Loaded Key: {}", tsk.fingerprint());
    // let ppr = PacketParser::from_file(&pgp_key_path).unwrap();
    // let mut public_key = "".to_string();
    // for certo in CertParser::from(ppr) {
    //     match certo {
    //         Ok(cert) => {
    //             info!("Key: {}", cert.fingerprint());
    //             public_key = String::from_utf8(cert.armored().to_vec().unwrap()).unwrap();
    //             debug!("public Key: {}", public_key);
    //             for ua in cert.userids() {
    //                 info!("  User ID: {}", ua.userid());
    //             }
    //         }
    //         Err(err) => {
    //             error!("Error reading keyring: {}", err);
    //         }
    //     }
    // }

    let (cache_r, raw_cache_w) = evmap::new();
    let cache_w = Arc::new(Mutex::new(raw_cache_w));
    // Start HTTP server
    HttpServer::new(move || {
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
                keyring: public_key.clone(),
            }))
            .wrap(middleware::Logger::default())
            //.service(web::resource("/hello").route(web::get().to(routes::index)))
            //.service(web::resource("/hello/{name}").route(web::get().to(routes::index)))
            .service(web::resource("/nas").route(web::get().to(routes::syno)))
            .service(web::resource("/nas").route(web::post().to(routes::syno)))
            //.service(web::resource("/package").route(web::get().to(routes::list_packages)))
            //.service(web::resource("/package/{id}").route(web::get().to(routes::get_package_version)))
            .service(auth::login)
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
                    // .service(auth::profile)
                    .service(user::get_all)
                    .service(user::delete)
                    .service(build::delete)
                    .service(build::active)
                    .service(build::delete_id)
                    .service(architecture::post)
                    .service(architecture::delete)
                    .service(architecture::get_all)
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
    })
    .bind(&bind)?
    .run()
    .await
}
