use crate::schema::*;
use crate::DbConn;
use diesel::prelude::*;

mod icon;
mod screenshot;

pub use self::icon::DbIcon;
pub use self::screenshot::DbScreenshot;

use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "architecture"]
pub struct DbArchitecture {
    pub id: u64,
    pub code: String,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[belongs_to(DbFirmware, foreign_key = "firmware_id")]
#[table_name = "build"]
pub struct DbBuild {
    pub id: u64,
    pub package_id: u64,
    pub firmware_id: u64,
    pub publisher_user_id: Option<u64>,
    pub checksum: Option<String>,
    pub exec_size: i32,
    pub path: String,
    pub md5: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[primary_key(build_id, architecture_id)]
#[table_name = "build_architecture"]
pub struct DbBuildArchitecture {
    pub build_id: u64,
    pub architecture_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[primary_key(language_id, version_id)]
#[table_name = "displayname"]
pub struct DbDisplayName {
    pub version_id: u64,
    pub language_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "language"]
pub struct DbLanguage {
    pub id: u64,
    pub code: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[table_name = "download"]
pub struct DbDownload {
    pub id: u64,
    pub build_id: u64,
    pub architecture_id: u64,
    pub firmware_build: u64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: u64,
    pub version: String,
    pub build: u64,
}

//
#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[primary_key(language_id, version_id)]
#[table_name = "description"]
pub struct DbDescription {
    pub version_id: u64,
    pub language_id: u64,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "package"]
pub struct DbPackage {
    pub id: u64,
    pub author_user_id: Option<u64>,
    pub name: String,
    pub insert_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(package_id, user_id)]
#[table_name = "package_user_maintainer"]
pub struct DbPackageUserMaintainer {
    pub package_id: u64,
    pub user_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "role"]
pub struct DbRole {
    pub id: u64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "service"]
pub struct DbService {
    pub id: u64,
    pub code: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "user"]
pub struct DbUser {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub api_key: Option<String>,
    pub github_access_token: Option<String>,
    pub active: bool,
    pub confirmed_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(user_id, role_id)]
#[table_name = "user_role"]
pub struct DbUserRole {
    pub user_id: u64,
    pub role_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[table_name = "version"]
pub struct DbVersion {
    pub id: u64,
    pub package_id: u64,
    pub ver: u32,
    pub upstream_version: String,
    pub changelog: Option<String>,
    pub report_url: Option<String>,
    pub distributor: Option<String>,
    pub distributor_url: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_url: Option<String>,
    pub dependencies: Option<String>,
    pub conf_dependencies: Option<String>,
    pub conflicts: Option<String>,
    pub conf_conflicts: Option<String>,
    pub install_wizard: Option<bool>,
    pub upgrade_wizard: Option<bool>,
    pub startable: Option<bool>,
    pub license: Option<String>,
    pub insert_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(version_id, package_id)]
#[table_name = "version_service_dependency"]
pub struct DbVersionServiceDependency {
    pub version_id: u64,
    pub package_id: u64,
}

// #[derive(Serialize)]
// pub struct Package {
//     beta: Option<bool>,
//     changelog: Option<String>,
//     conflictpkgs: Option<String>,
//     deppkgs: Option<String>,
//     desc: Option<String>,
//     distributor: String,
//     distributor_url: String,
//     dname: Option<String>,
//     download_count: usize,
//     link: Option<String>,
//     maintainer: &'static str,
//     package: Option<String>,
//     qinst: bool,
//     qstart: bool,
//     qupgrade: bool,
//     recent_download_count: usize,
//     thumbnail: Vec<String>,
//     thumbnail_retina: Vec<String>,
//     version: Option<String>,
// }
#[derive(Serialize, Queryable, Debug, Clone)]
pub struct MyPackage {
    pub package_id: u64,
    pub version_id: u64,
    pub beta: bool,
    //     conflictpkgs: Option<String>,
    //     deppkgs: Option<String>,
    pub changelog: Option<String>,
    pub desc: Option<String>,
    pub distributor: Option<String>,
    pub distributor_url: Option<String>,
    pub dname: Option<String>,
    // download_count: u64,
    pub link: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_url: Option<String>,
    pub package: String,
    pub qinst: Option<bool>,
    pub qstart: Option<bool>,
    pub qupgrade: Option<bool>,
    // recent_download_count: u64,
    // thumbnail: Vec<String>,
    // thumbnail: Option<String>,
    pub upstream_version: String,
    pub revision: u32,
    pub md5: String,
    pub size: i32,
}

impl DbPackage {
    // pub fn to_json(&self) -> Value {
    //     json!({
    //         "Id": self.uuid,
    //         "OrganizationId": self.org_uuid,
    //         "Name": self.name,
    //         "Object": "collection",
    //     })
    // }
    pub fn get_packages(lang: &String, arch: &String, build: u64, beta: bool, major: u8, _micro: u8, minor: u8, conn: &DbConn) -> Vec<MyPackage> {
        let firmware = format!("{}.{}", major, minor);

        let language_id = language::table
            .filter(language::code.eq(lang))
            .select(language::id)
            .first::<u64>(&**conn)
            .expect("Error loading language");

        let architecture_id = architecture::table
            .filter(architecture::code.eq(arch))
            .select(architecture::id)
            .first::<u64>(&**conn)
            .expect("Error loading architecture");

        // let _icons = icon::table
        //     .filter(icon::version_id.eq(1))
        //     .select((icon::path, icon::size))
        //     .load::<(String, String)>(&**conn)
        //     .expect("Error loading icons");

        // sql("group_concat(icon::path) as images")

        let mut q = package::table
            .inner_join(
                version::table
            //         .left_outer_join(version::table.on(version::id.eq(version::id).and(version::ver.gt(version::ver))))
            //         // .filter(version::id.is_null())
                    .left_join(description::table)
                    .left_join(displayname::table)
            )
            .inner_join(
                build::table.inner_join(firmware::table).inner_join(
                    build_architecture::table.on(build_architecture::build_id
                        .eq(build::id)
                        .and(build_architecture::architecture_id.eq(architecture_id))),
                ),
            )
            .filter(build::active.eq(true))
            .filter(firmware::version.eq(firmware))
            .filter(firmware::build.ge(build))
            .filter(description::language_id.eq(language_id))
            .filter(displayname::language_id.eq(language_id))
            .into_boxed(); // http://docs.diesel.rs/diesel/query_dsl/trait.QueryDsl.html#method.into_boxed
            if !beta {
                q=q.filter(version::report_url.is_not_null());
            }
            q.select((
                package::id,
                version::id,
                version::report_url.is_not_null(), // beta
                version::changelog,
                description::desc.nullable(),
                version::distributor,
                version::distributor_url,
                displayname::name.nullable(),
                build::path.nullable(),
                version::maintainer,
                version::maintainer_url,
                package::name,
                version::install_wizard,
                version::startable,
                version::upgrade_wizard,
                // icon::path.nullable(),
                version::upstream_version,
                version::ver,
                build::md5,
                build::extract_size,
            ))
            .load::<MyPackage>(&**conn)
            .expect("Error loading packages")
    }
}