use crate::schema::*;
use anyhow::{Context, Result};
use diesel::prelude::*;
use diesel::{sql_query};
use diesel::sql_types::{
    BigInt, Bool, Integer, Text, Nullable
};

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

#[derive(Serialize, QueryableByName, Debug, Clone)]
pub struct MyPackage {
    #[sql_type="diesel::mysql::types::Unsigned<BigInt>"]
    pub package_id: u64,
    #[sql_type="diesel::mysql::types::Unsigned<BigInt>"]
    pub version_id: u64,
    #[sql_type = "Bool"]
    pub beta: bool,
    #[sql_type = "Nullable<Text>"]
    pub conflictpkgs: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub deppkgs: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub changelog: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub desc: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub distributor: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub distributor_url: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub dname: Option<String>,
    // download_count: u64,
    #[sql_type = "Nullable<Text>"]
    pub link: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub maintainer: Option<String>,
    #[sql_type = "Nullable<Text>"]
    pub maintainer_url: Option<String>,
    #[sql_type = "Text"]
    pub package: String,
    #[sql_type = "Nullable<Bool>"]
    pub qinst: Option<bool>,
    #[sql_type = "Nullable<Bool>"]
    pub qstart: Option<bool>,
    #[sql_type = "Nullable<Bool>"]
    pub qupgrade: Option<bool>,
    // recent_download_count: u64,
    #[sql_type = "Text"]
    pub upstream_version: String,
    #[sql_type = "diesel::mysql::types::Unsigned<Integer>"]
    pub revision: u32,
    #[sql_type = "Text"]
    pub md5: String,
    #[sql_type = "Integer"]
    pub size: i32,
}

impl DbPackage {
    pub fn get_packages(
        lang: &String,
        arch: &String,
        build: u64,
        beta: bool,
        major: u8,
        _micro: u8,
        minor: u8,
        conn: &MysqlConnection,
    ) -> Result<Vec<MyPackage>> {
        let firmware = format!("{}.{}", major, minor);

        let language_id_fallback_eng: u64 = 1;
        let language_id = language::table
            .filter(language::code.eq(lang))
            .select(language::id)
            .first::<u64>(conn)
            .unwrap_or(language_id_fallback_eng);

        let architecture_id = architecture::table
            .filter(architecture::code.eq(arch))
            .select(architecture::id)
            .first::<u64>(conn)
            .context("Error loading architecture from DB")?; // todo return 404

        let query = sql_query(r#"
                SELECT
                `package`.`id` AS package_id,
                `version`.`id` AS version_id,
                (CASE WHEN `version`.`report_url` <> '' THEN true ELSE false END) AS beta,
                `version`.`conflicts` AS conflictpkgs,
                `version`.`dependencies` AS deppkgs,
                `version`.`changelog`,
                `description`.`description` AS "desc",
                `version`.`distributor`,
                `version`.`distributor_url`,
                `displayname`.`displayname` AS dname,
                `build`.`path` AS link,
                `version`.`maintainer`,
                `version`.`maintainer_url`,
                `package`.`name` AS package,
                `version`.`install_wizard` AS qinst,
                `version`.`startable` AS qstart,
                `version`.`upgrade_wizard` AS qupgrade,
                `version`.`upstream_version`,
                `version`.`version` AS revision,
                `build`.`md5`,
                `build`.`extract_size` AS size

                FROM
                (
                    (
                    `package`
                    INNER JOIN (
                        (
                        (
                            `version`
                            LEFT OUTER JOIN `description` ON `description`.`version_id` = `version`.`id`
                            AND `description`.`language_id` = CASE WHEN EXISTS (
                            SELECT 1
                            FROM `description`
                            WHERE `description`.`language_id` = ?
                            ) THEN ? ELSE 1 END
                        )
                        LEFT OUTER JOIN `displayname` ON `displayname`.`version_id` = `version`.`id`
                        AND `displayname`.`language_id` = CASE WHEN EXISTS (
                            SELECT  1
                            FROM  `displayname`
                            WHERE  `displayname`.`language_id` = ?
                        ) THEN ? ELSE 1 END
                        )
                        INNER JOIN (
                        SELECT `version`.`id`, MAX(`version`.`version`) `version`, `package_id`
                        FROM `version`
                        GROUP BY `version`.`package_id`
                        ) ver ON `version`.`package_id` = `ver`.`package_id`
                        AND `version`.`version` = `ver`.`version`
                    ) ON `version`.`package_id` = `package`.`id`
                    )
                    INNER JOIN (
                    (
                        `build`
                        INNER JOIN `firmware` ON `firmware`.`id` = `build`.`firmware_id`
                        AND `firmware`.`version` = ?
                    )
                    INNER JOIN `build_architecture` ON `build_architecture`.`build_id` = `build`.`id`
                    AND `build_architecture`.`architecture_id` = ?
                    ) ON `build`.`package_id` = `package`.`id`
                )
                WHERE `build`.`active` = true
                AND `firmware`.`build` >= ?
                AND (? OR `version`.`report_url` = '')
            "#);

        let packages = query
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
        .bind::<Text, _>(firmware)
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(architecture_id)
        .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(build)
        .bind::<Bool, _>(beta)
        .load::<MyPackage>(conn)
            .context("Error loading packages from DB")?;
        Ok(packages)
        // println!("{:?}", diesel::debug_query::<diesel::mysql::Mysql, _>(&q));
        // let s = String::new();
        // let os = Some(String::new());
        // let mut v = Vec::new();
        // let ob = Some(false);
        // v.push(MyPackage{package_id:1,version_id:1,beta:false,conflictpkgs:None,deppkgs:None,changelog:os.clone(),desc:os.clone(),distributor:os.clone(),distributor_url:os.clone(),dname:os.clone(),link:os.clone(),maintainer:os.clone(),maintainer_url:os.clone(),package:s.clone(),qinst:ob,qstart:ob,qupgrade:ob,upstream_version:s.clone(),revision:1,md5:s.clone(),size:300});
        // v
    }
}
