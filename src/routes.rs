#![allow(unused)]

use crate::URL;
use diesel::{self, prelude::*};
use rocket::request::LenientForm;
use rocket_contrib::json::Json;
use crate::models::*;
use crate::DbConn;

const KEYRING: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----\nVersion: GnuPG v1\n\nmQENBFRhPdoBCADMWckT2GJRrRcuNXuCBNp3oSC7tlQxa1EN81HhlX2Tqs7tKezC\nvgGCB69jWQmfB5BKdWcznS98bLZB4Iy2RjU8vtkI0/6AceovCytMXW0d6HE8frlf\n6gkWKylRbD3fBE+qpHOEwpV5MTEgF8UTM9cPzupY6ggm/6fSDxXqYRZQHfnAFoLE\nXSzMtdUyY0w4a1CapfVRa060XRNLvacu6+1XVksJVZbuChg3/zDhtYZuvbuXxwfA\n/sZem9OW85roUgsYE3cL8m4iexZHMIbWBO5mj7LoYgb33vF7T15yGUjWADMbBkQx\nYFBg6q48Nc81taFHRWpIIXe1s1ZTxyBQL0hjABEBAAG0NFN5bm9Db21tdW5pdHkg\nKE9mZmljaWFsKSA8Y29udGFjdEBzeW5vY29tbXVuaXR5LmNvbT6JATgEEwECACIF\nAlRhPdoCGwMGCwkIBwMCBhUIAgkKCwQWAgMBAh4BAheAAAoJENDC8YaUoLiOtJkI\nAKpGpoKrmkzSFEhSj+tbTx/EdsrQu+9Q32H51EZlM0gSn1rzjPBsg20Uh3JoK2gO\nDrWtcL6wSgd6Vp9ScwcjH/GQ6fh5/AIcXl1oW/Z31ZiqGxJmdT1EwdqYZdN+bv8K\nF4rezHKwlUAsq4jHvwnmOfjqJzn4Gpbf0diajLBNMmZY0uOe8Q0s1IqNkrtw0zWD\nimZqYTrktnpm8YFDUe1xo6qRNdqVXn5lddfrO4hPDP2/hzgZ6l0Gnl4P0ZFYAGo9\nQITV2BqBbBpMYff/yF0yxbSQgCu93J3FtMe3qK6mu2lclSDEFs+abX0NIbUOTv4l\nAus7c0ZXjYOZNLAYY+RAXsO5AQ0EVGE92gEIAJw1AdFZ1MXlU+JeCLqes8LV3Grv\nhTvTRWTd7Pi3W+qoaGkeTCfc9Jxf5PgFr0s5ZJrXD6f/JF73JSFwuHrGacSAR28/\nnPcLZPN5JYDipWmSdoa672lEeDJ+Zr2f2jtFs0CTXbyTyVSZnoDtL5j7a3BtlJ6N\nw2FaGVeqto7qfkudizEnoNcokmeAD0EpajCq2L0ZO6QxTP8q3gVoffQK6UTOublJ\nHj1T5A1ZH+hgVmjAsQ7AOh3ElRml+lkd3j0luYiuMiz8ol3GHjTQ0C5GnbWka3LH\nnrgU75kJduGtngEnmR6dBZPR47ImjsX5cQ7JWrJLSrWc907+7vcb6cAwYcUAEQEA\nAYkBHwQYAQIACQUCVGE92gIbDAAKCRDQwvGGlKC4jgEUB/9jwTxRbVGKjVyO9ZdP\nYR5seJU0R3ZUKZa5+Qv7BXPSaBS6nCHejxOd9Jg8zYafVTDdCYdvDfNrKnhhKOC9\n637WGNd/T7LfPH0fp7KHKv+QJ15LhleMpcsKVt8+32px7jepAltD6drNTATkDyST\nQz5PMrVZLkhZo2zu/I8sfj/SAd0mtoBBpRfA3Xt9AWCMqaWcSM9nmz3awzJopVY3\nUXnX9p13B4op2wyPnoW0j1GdBRv/Ky2kOYE++AczGwhbPos2fD3Ulg4aIKspgZ5f\nsvlMBaG/AAd69IVvWQYqlUvyB1v6i1Trl6Ti2sNd6eAphNAJeQGCTcU3w6ibvAq5\nyshz\n=pO8s\n-----END PGP PUBLIC KEY BLOCK-----";

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
pub struct SynoRequest {
    arch: String,                           // apollolake
    build: u64,                           // 24922
    language: String,                       // enu
    major: u8,                              // 6
    micro: u8,                              // 2
    minor: u8,                              // 2
    nano: Option<u8>,                       // 4
    package_update_channel: Option<String>, // beta/stable
    timezone: Option<String>,               // London
    unique: Option<String>,                 // synology_apollolake_418play
}

// fn is_false (b: bool) -> bool {
//     !b
// }

extern crate serde_with;
#[serde_with::skip_serializing_none]
#[derive(Serialize)]
pub struct Package {
    // #[serde(skip_serializing_if = "is_false")]
    pub beta: bool,
    pub changelog: Option<String>,
    pub conflictpkgs: Option<String>,
    pub deppkgs: Option<String>,
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub snapshot: Vec<String>,
    pub distributor: String,
    pub distributor_url: String,
    pub dname: Option<String>,
    pub download_count: usize,
    pub link: String,
    pub maintainer: String,
    pub maintainer_url: String,
    pub package: String,
    pub qinst: bool,
    pub qstart: bool,
    pub qupgrade: bool,
    pub recent_download_count: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub thumbnail_retina: Vec<String>,
    pub version: String,
    pub md5: Option<String>,
    pub size: Option<i32>,
}

impl Package {
    fn new(
        beta: bool,
        changelog: Option<String>,
        desc: Option<String>,
        distributor: String,
        distributor_url: String,
        dname: Option<String>,
        link: String,
        maintainer: String,
        maintainer_url: String,
        package: String,
        qinst: bool,
        qstart: bool,
        qupgrade: bool,
        version: String,
        md5: Option<String>,
        size: Option<i32>,
    ) -> Self {
        Package {
            beta: false,
            changelog,
            conflictpkgs: None,
            deppkgs: None,
            desc,
            snapshot: Vec::new(),
            distributor,
            distributor_url,
            dname,
            download_count: 0,
            link,
            maintainer,
            maintainer_url,
            package,
            qinst,
            qstart,
            qupgrade,
            recent_download_count: 0,
            thumbnail: Vec::new(),
            thumbnail_retina: Vec::new(),
            version,
            md5,
            size,
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Package {
            beta: false,
            changelog: None,
            conflictpkgs: None,
            deppkgs: None,
            desc: None,
            snapshot: Vec::new(),
            distributor: "Syno Community".to_string(),
            distributor_url: "https://synocommunity.com/".to_string(),
            dname: None,
            download_count: 0,
            link: String::new(),
            maintainer: "Syno Community".to_string(),
            maintainer_url: "https://synocommunity.com/".to_string(),
            package: String::new(),
            qinst: false,
            qstart: false,
            qupgrade: true,
            recent_download_count: 0,
            thumbnail: Vec::new(),
            thumbnail_retina: Vec::new(),
            version: String::new(),
            md5: None,
            size: None,
        }
    }
}

#[derive(Serialize, Default)]
pub struct SynoResponse {
    keyrings: Option<Vec<String>>,
    packages: Vec<Package>,
}
impl SynoResponse {
    fn set_key(&mut self, key: String) -> &Self {
        let mut k = self.keyrings.clone().unwrap_or(Vec::new());
        k.push(key);
        self.keyrings = Some(k);
        self
    }
}
pub fn get_packages_for_device_lang(
    conn: DbConn,
    lang: &String,
    arch: &String,
    build: u64,
    package_update_channel: &Option<String>,
    major: u8,
    micro: u8,
    minor: u8,
) -> SynoResponse {
    let mut beta = false;
    if let Some(package_update_channel) = package_update_channel {
        if (package_update_channel == "beta") {
            beta = true;
        }
    }

    let mut sr = SynoResponse {
        packages: Vec::new(),
        ..Default::default()
    };
    sr.set_key(KEYRING.to_string());

    let packages = DbPackage::get_packages(&lang, &arch, build, beta, major, micro, minor, &conn);
    // println!("{}", serde_json::to_string_pretty(&packages).unwrap());

    for package in packages.iter() {
        let mut p = Package::new(
            package.beta,
            package.changelog.clone(),
            package.desc.clone(),
            package.distributor.clone().unwrap_or(String::new()),
            package.distributor_url.clone().unwrap_or(String::new()),
            package.dname.clone(),
            format!(
                "{}/{}/{}/{}",
                URL.to_string(),
                package.package.clone(),
                package.revision,
                package.link.clone().unwrap_or(String::new()),
            ),
            package.maintainer.clone().unwrap_or(String::new()),
            package.maintainer_url.clone().unwrap_or(String::new()),
            package.package.clone(),
            package.qinst.unwrap_or(false),
            package.qstart.unwrap_or(false),
            package.qupgrade.unwrap_or(false),
            format!("{}-{}", package.upstream_version, package.revision),
            Some(package.md5.clone()),
            Some(package.size),
        );
        p.thumbnail = DbIcon::paths(
            DbIcon::from_version(package.version_id, &conn),
            format!(
                "{}/{}",
                package.dname.clone().unwrap_or(String::new()),
                package.revision
            ),
        );
        p.thumbnail_retina = DbIcon::retina_from_version(package.version_id, &conn)
            .iter()
            .map(|icon| {
                format!(
                    "{}/{}/{}/{}",
                    URL.to_string(),
                    package.package.clone(),
                    package.revision,
                    icon.path.clone()
                )
            })
            .collect::<Vec<_>>();
        p.snapshot = DbScreenshot::from_package(package.package_id, &conn)
            .iter()
            .map(|screenshot| {
                format!(
                    "{}/{}/{}/{}",
                    URL.to_string(),
                    package.package.clone(),
                    package.revision,
                    screenshot.path.clone()
                )
            })
            .collect::<Vec<_>>();
        sr.packages.push(p);
    }
    // sr.packages.push(Package::default());
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

pub fn int_to_float(a: u32, b: u32) -> f32 {
    let after_decimal_place: f32 = a as f32 / 100.0;
    b as f32 + after_decimal_place
}

#[get("/package")]
pub fn list_packages(connection: DbConn) -> Json<Vec<DbPackage>> {
    use crate::schema::package;

    let packages = package::table
        // .filter(package::name.eq("dnscrypt-proxy"))
        .load::<DbPackage>(&connection.0)
        .expect("Error loading package");
    Json(packages)

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
