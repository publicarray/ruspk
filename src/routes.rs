#![allow(unused)]

use diesel::{self, prelude::*};

use rocket::request::LenientForm;
use rocket_contrib::json::Json;

use crate::models::{DbPackage, DbVersion};
// use crate::schema;
use crate::DbConn;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
pub struct SynoRequest {
    arch: String,                           // apollolake
    build: usize,                           // 24922
    language: String,                       // enu
    major: u8,                              // 6
    micro: u8,                              // 2
    minor: u8,                              // 2
    nano: Option<u8>,                       // 4
    package_update_channel: Option<String>, // beta
    timezone: Option<String>,               // London
    unique: Option<String>,                 // synology_apollolake_418play
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

#[derive(Serialize, Default)]
pub struct SynoResponse {
    keyrings: Option<Vec<String>>,
    packages: Vec<Package>,
}

#[get("/?<synorequest..>")]
pub fn syno(synorequest: LenientForm<SynoRequest>) -> Json<SynoResponse> {
    let mut sr = SynoResponse {
        packages: Vec::new(),
        ..Default::default()
    };
    use crate::schema::package::dsl::*;
    use crate::schema::version::dsl::*;
    sr.packages.push(Package::new());
    Json(sr)
#[get("/package")]
pub fn list_packages(conn: DbConn) -> Json<Vec<(DbPackage, Vec<DbVersion>)>> {
    use crate::schema::package::dsl::*;

    let p = package
        .load::<DbPackage>(&conn.0)
        .expect("Error loading package");
    // let p = package.find(1).get_result::<DbPackage>(&conn.0).expect("Error loading package");
    let versions = DbVersion::belonging_to(&p)
        .load::<DbVersion>(&conn.0)
        .expect("Error loading version")
        .grouped_by(&p);
    let data = p.into_iter().zip(versions).collect::<Vec<_>>();
    Json(data)
}

#[get("/package/<num>")]
pub fn get_package_version(conn: DbConn, num: u64) -> Json<Vec<DbVersion>> {
    use crate::schema::version::dsl::*;
    Json(
        version
            .filter(package_id.eq(num))
            .load(&*conn.0)
            .expect("Error loading version"),
    )
}
