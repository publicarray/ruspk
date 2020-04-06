#![allow(unused)]
use diesel::{self, prelude::*};

use rocket_contrib::json::Json;
use rocket::request::LenientForm;

use crate::models::{DbPackage, DbVersion};
// use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
pub struct SynoRequest {
    arch: String, // apollolake
    build: usize, // 24922
    language: String, // enu
    major: u8, // 6
    micro: u8, // 2
    minor: u8, // 2
    nano: Option<u8>, // 4
    package_update_channel: Option<String>, // beta
    timezone: Option<String>, // London
    unique: Option<String>, // synology_apollolake_418play
}

#[derive(Serialize)]
pub struct Package {
    beta: Option<bool>,
    changelog: Option<String>,
    conflictpkgs: Option<String>,
    deppkgs: Option<String>,
    desc: Option<String>,
    distributor: String,
    distributor_url: String,
    dname: Option<String>,
    download_count: usize,
    link: Option<String>,
    maintainer: String,
    package: Option<String>,
    qinst: bool,
    qstart: bool,
    qupgrade: bool,
    recent_download_count: usize,
    thumbnail: Vec<String>,
    thumbnail_retina: Vec<String>,
    version: Option<String>,
}

impl Package {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Package {
    fn default() -> Self {
        Package {
            beta: None,
            changelog: None,
            conflictpkgs: None,
            deppkgs: None,
            desc: None,
            distributor: "Syno Community".to_string(),
            distributor_url: "https://synocommunity.com/".to_string(),
            dname: None,
            download_count: 0,
            link: None,
            maintainer: "Syno Community".to_string(),
            package: None,
            qinst: false,
            qstart: false,
            qupgrade: true,
            recent_download_count: 0,
            thumbnail: [].to_vec(),
            thumbnail_retina: [].to_vec(),
            version: None,
        }
    }
}

#[derive(Serialize)]
pub struct SynoResponse {
    keyrings: Option<Vec<String>>,
    packages: Vec<Package>,

}
#[get("/?<synorequest..>")]
pub fn syno(synorequest: LenientForm<SynoRequest>) -> Json<SynoResponse> {
    let mut sr = SynoResponse{ keyrings: None, packages: Vec::new() };
    sr.packages.push(Package::new());
    Json(sr)
#[get("/package")]
pub fn list_packages(conn: DbConn) -> Result<Json<Vec<DbPackage>>, String> {
    use crate::schema::package::dsl::*;

    package.load(&conn.0).map_err(|err| -> String {
        println!("Error querying package: {:?}", err);
        "Error querying package from the database".into()
    }).map(Json)
}

#[get("/package/<num>")]
pub fn get_package_version(conn: DbConn, num: u64) -> Result<Json<Vec<DbVersion>>, String> {
    use crate::schema::version::dsl::*;
    version.filter(package_id.eq(num)).load(&*conn.0).map_err(|err| -> String {
        println!("Error querying package: {:?}", err);
        "Error querying package from the database".into()
    }).map(Json)
}
