#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// use rocket_contrib::databases::diesel;
pub mod cors;
pub mod models;
pub mod routes;
pub mod schema;
// use schema::package;
// pub mod schema;
// pub mod models;

const URL: &str = "http://packages.synocommunity.com";

#[database("mysql")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::syno,
                routes::syno_post,
                routes::get_package_version,
                routes::list_packages,
            ],
        )
        .attach(cors::CorsFairing)
        .attach(DbConn::fairing())
        .launch();
}
