use crate::schema::*;

mod icon;
mod screenshot;
mod architecture;
mod language;
mod build;
mod firmware;
mod displayname;
mod download;
mod description;
mod version;
mod user;
mod role;
mod service;
mod package;

pub use self::icon::DbIcon;
pub use self::screenshot::DbScreenshot;
pub use self::architecture::DbArchitecture;
pub use self::language::DbLanguage;
pub use self::build::DbBuild;
pub use self::firmware::DbFirmware;
pub use self::displayname::DbDisplayName;
pub use self::download::DbDownload;
pub use self::description::DbDescription;
pub use self::version::DbVersion;
pub use self::user::DbUser;
pub use self::role::DbRole;
pub use self::service::DbService;
pub use self::package::DbPackage;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[primary_key(build_id, architecture_id)]
#[table_name = "build_architecture"]
pub struct DbBuildArchitecture {
    pub build_id: u64,
    pub architecture_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(package_id, user_id)]
#[table_name = "package_user_maintainer"]
pub struct DbPackageUserMaintainer {
    pub package_id: u64,
    pub user_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(user_id, role_id)]
#[table_name = "user_role"]
pub struct DbUserRole {
    pub user_id: u64,
    pub role_id: u64,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(version_id, package_id)]
#[table_name = "version_service_dependency"]
pub struct DbVersionServiceDependency {
    pub version_id: u64,
    pub package_id: u64,
}
