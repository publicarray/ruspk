use chrono::NaiveDateTime;

use crate::schema::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
#[table_name = "language"]
pub struct DbLanguage {
    pub id: u64,
    pub code: String,
    // pub code: [char;3],
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
#[table_name = "architecture"]
pub struct DbArchitecture {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
#[table_name = "package"]
pub struct DbPackage {
    pub id: u64,
    pub author_user_id: Option<u64>,
    pub name: String,
    pub insert_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
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

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[table_name = "description"]
pub struct DbDescription {
    pub id: u64,
    pub language_id: u64,
    pub version: u32,
    pub desc: String,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[primary_key(language_id, version_id)]
#[table_name = "displayname"]
pub struct DbDisplayName {
    pub version_id: u64,
    pub language_id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: u64,
    pub version: f32,
    pub build: u64,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[table_name = "icon"]
pub struct DbIcon {
    pub id: u64,
    pub version_id: u64,
    pub size: u16,
    pub path: String,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[belongs_to(DbFirmware, foreign_key = "firmware_id")]
#[table_name = "build"]
pub struct DbBuild {
    pub id: u64,
    pub package_id: u64,
    pub firmware_id: u64,
    pub publisher_user_id: u64,
    pub checksum: String,
    pub exec_size: u64,
    pub path: String,
    pub md5: String,
    pub insert_date: Option<NaiveDateTime>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Queryable, Associations, Debug)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[table_name = "build_architecture"]
pub struct DbBuildArchitecture {
    pub build_id: u64,
    pub architecture_id: u64,
}
