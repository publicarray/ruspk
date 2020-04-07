#![allow(unused)]

use diesel::{self, prelude::*};

use rocket::request::LenientForm;
use rocket_contrib::json::Json;

use crate::models::*;
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
            distributor: String::from("Syno Community"),
            distributor_url: String::from("https://synocommunity.com/"),
            dname: None,
            download_count: 0,
            link: None,
            maintainer: String::from("Syno Community"),
            package: None,
            qinst: false,
            qstart: false,
            qupgrade: true,
            recent_download_count: 0,
            thumbnail: Vec::new(),
            thumbnail_retina: Vec::new(),
            version: None,
        }
    }
}

#[derive(Serialize, Default)]
pub struct SynoResponse {
    keyrings: Option<Vec<String>>,
    packages: Vec<Package>,
}

pub fn get_packages_for_device_lang(
    conn: DbConn,
    lang: &String,
    arch: &String,
    build: usize,
    package_update_channel: &Option<String>,
    major: u8,
    micro: u8,
    minor: u8,
) -> SynoResponse {
    let beta = false;
    if let Some(package_update_channel) = package_update_channel {
        if (package_update_channel == "beta") {
            let beta = true;
        }
    }

    ////
    use crate::schema::description::dsl::*;
    use crate::schema::displayname::dsl::*;
    use crate::schema::language::dsl::*;
    use crate::schema::package::dsl::*;
    use crate::schema::package::dsl::*;
    use crate::schema::version::dsl::*;

    let languages = language
        .load::<DbLanguage>(&conn.0)
        .expect("Error loading languages");

    let descriptions = description
        .load::<DbDescription>(&conn.0)
        // .inner_join(language)
        .expect("Error loading description");

    let packages = package
        .load::<DbPackage>(&conn.0)
        .expect("Error loading package");
    // println!("{:?}", packages);

    // let p = package.find(1).get_result::<DbPackage>(&conn.0).expect("Error loading package");
    let versions = DbVersion::belonging_to(&packages)
        .load::<DbVersion>(&conn.0)
        .expect("Error loading version")
        .grouped_by(&packages);

    // let description = DbDescription::belonging_to(&languages)
    //     .load::<DbDescription>(&conn.0)
    //     .expect("Error loading description");

    // let displayName = DbDisplayName::belonging_to(&versions, &languages)
    // // let displayName = DbDisplayName::belonging_to(&versions, &languages)
    //     .load::<DbDisplayName>(&conn.0)
    //     .expect("Error loading displayname");

    let data = packages.into_iter().zip(versions).collect::<Vec<_>>();
    // let data = packages.into_iter().zip(displayName).collect::<Vec<_>>();
    println!("{:?}", data);
    let mut sr = SynoResponse {
        packages: Vec::new(),
        ..Default::default()
    };

    sr.packages.push(Package::new());
    // sr.packages.push(Package::new(..Default::default()));
    return sr;
}

// For old Synology devices
#[post("/", data = "<synorequest>")]
pub fn syno_post(synorequest: LenientForm<SynoRequest>, conn: DbConn) -> Json<SynoResponse> {
    Json(get_packages_for_device_lang(
        conn,
        &synorequest.language,
        &synorequest.arch,
        synorequest.build,
        &synorequest.package_update_channel,
        synorequest.major,
        synorequest.micro,
        synorequest.minor,
    ))
}

#[get("/?<synorequest..>")]
pub fn syno(synorequest: LenientForm<SynoRequest>, conn: DbConn) -> Json<SynoResponse> {
    Json(get_packages_for_device_lang(
        conn,
        &synorequest.language,
        &synorequest.arch,
        synorequest.build,
        &synorequest.package_update_channel,
        synorequest.major,
        synorequest.micro,
        synorequest.minor,
    ))
}

// ?package_update_channel=beta&unique=synology_apollolake_418play&build=24922&language=enu&major=6&micro=2&arch=apollolake&minor=2&timezone=London&nano=4
// #[get("/?package_update_channel&<package_update_channel>")]
// fn hello(package_update_channel: Option<bool>) -> bool {
//     format!("Hello, {}!", package_update_channel.as_str())
// }

// #[post("/", data = "<form_data>")]
// fn login(form_data: Form<UserLogin>) -> String {
//    format!("Hello, {}!", form_data.name)
// }

// #[post("/Package", data = "<page_view>")]
// pub fn create_page_view(
//     conn: DbConn,
//     page_view: Json<InsertablePageView>,
// ) -> Result<String, String> {
//     let inserted_rows = diesel::insert_into(schema::pageviews::table)
//         .values(&page_view.0)
//         .execute(&conn.0)
//         .map_err(|err| -> String {
//             println!("Error inserting row: {:?}", err);
//             "Error inserting row into database".into()
//         })?;

//     Ok(format!("Inserted {} row(s).", inserted_rows))
// }

#[get("/package")]
pub fn list_packages(connection: DbConn) -> Json<Vec<DbPackage>> {
    let query_package_update_channel = "beta";
    let query_language = "enu";
    let query_major = 6;
    let query_micro = 2;
    let query_arch = "apollolake";
    let query_minor = 2;
    let query_build = 24922;
    let query_version = format!(
        "{}.{}.{}-{}",
        query_major, query_micro, query_minor, query_build
    );

    use crate::schema::architecture;
    use crate::schema::description;
    use crate::schema::language;
    use crate::schema::displayname;
    use crate::schema::package;
    use crate::schema::version;

    let lang_id = language::table
        .select(language::id)
        .filter(language::code.eq(query_language))
        .load::<u64>(&connection.0)
        .expect("Error loading language");
    println!("{:?}", lang_id[0]);

    let arch_id = architecture::table
        .select(architecture::id)
        .filter(architecture::code.eq(query_arch))
        .load::<u64>(&connection.0)
        .expect("Error loading architecture");
    println!("{:?}", arch_id[0]);

    let descriptions = description::table
        .filter(description::language_id.eq(lang_id[0]))
        .load::<DbDescription>(&connection.0)
        .expect("Error loading description");
    println!("{:?}", descriptions);

    let displaynames = displayname::table
        .filter(displayname::language_id.eq(lang_id[0]))
        .load::<DbDisplayName>(&connection.0)
        .expect("Error loading displayname");
    println!("{:?}", displaynames);

    let versions = version::table
        .load::<DbVersion>(&connection.0)
        .expect("Error loading versions");
    println!("{:?}", versions);

    // let version = DbDescription::belonging_to(&versions)
    //     .load::<DbDescription>(&connection.0);
    // println!("{:?}", description);

    // println!("Displaying {} posts", results.len());
    // for p in &results {
    //     println!("{}", p.id);
    //     println!("----------\n");
    //     println!("{}", p.name);
    //     println!("----------\n");
    //     println!("{:#?}", p.insert_date);
    // }
    let results = package::table
        .filter(package::name.eq("dnscrypt-proxy"))
        .load::<DbPackage>(&connection.0)
        .expect("Error loading package");
    Json(results)
    // let packages = package::table
    //     .load::<DbPackage>(&connection.0)
    //     .expect("Error loading package");

    // let p = package
    //     .load::<DbPackage>(&connection.0)
    //     .expect("Error loading package");
    // // let p = package.find(1).get_result::<DbPackage>(&conn.0).expect("Error loading package");
    // let versions = DbVersion::belonging_to(&p)
    //     .load::<DbVersion>(&connection.0)
    //     .expect("Error loading version")
    //     .grouped_by(&p);
    // let data = p.into_iter().zip(versions).collect::<Vec<_>>();
    // Json(data)

    // Json(package.load(&conn.0).expect("Error loading package"))

    // println!("p: {:?}, version: {:?}", p, version);
    // package.load(&conn.0).map_err(|err| -> String {
    //     println!("Error querying package: {:?}", err);
    //     "Error querying package from the database".into()
    // }).map(Json)
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
