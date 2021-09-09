use crate::schema::*;
use crate::DbId;

mod architecture;
mod build;
mod description;
mod displayname;
mod download;
mod firmware;
mod icon;
#[cfg(feature = "postgres")]
mod icon_size_type;
mod language;

#[cfg(feature = "postgres")]
#[path = "package_pg.rs"]
mod package;
#[cfg(feature = "sqlite")]
#[path = "package_sqlite.rs"]
mod package;
#[cfg(feature = "mysql")]
#[path = "package_mysql.rs"]
mod package;
mod role;
mod screenshot;
mod service;
mod user;
mod version;

pub use self::architecture::*;
pub use self::build::*;
pub use self::description::*;
pub use self::displayname::*;
pub use self::download::*;
pub use self::firmware::*;
pub use self::icon::*;
#[cfg(feature = "postgres")]
pub use self::icon_size_type::{IconSize, IconSizeEnum};
pub use self::language::*;
pub use self::package::*;
pub use self::role::*;
pub use self::screenshot::*;
pub use self::service::*;
pub use self::user::*;
pub use self::version::*;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(Build, foreign_key = "build_id")]
#[belongs_to(BuildTmp, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[primary_key(build_id, architecture_id)]
#[table_name = "build_architecture"]
pub struct DbBuildArchitecture {
    pub build_id: DbId,
    pub architecture_id: DbId,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(BuildTmp, foreign_key = "build_id")]
#[primary_key(build_id, architecture_id)]
#[table_name = "build_architecture"]
pub struct BuildArchitecture {
    pub build_id: DbId,
    pub architecture_id: DbId,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(package_id, user_id)]
#[table_name = "package_user_maintainer"]
pub struct DbPackageUserMaintainer {
    pub package_id: DbId,
    pub user_id: DbId,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(user_id, role_id)]
#[table_name = "user_role"]
pub struct DbUserRole {
    pub user_id: DbId,
    pub role_id: DbId,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[primary_key(version_id, package_id)]
#[table_name = "version_service_dependency"]
pub struct DbVersionServiceDependency {
    pub version_id: DbId,
    pub package_id: DbId,
}
