use crate::models::DbArchitecture;
use crate::models::DbLanguage;
use crate::schema::*;
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Bool, Integer, Nullable, Text};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "package"]
pub struct DbPackage {
    pub id: u64,
    pub author_user_id: Option<u64>,
    pub name: String,
    pub insert_date: Option<NaiveDateTime>,
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
    ) -> Result<Vec<DBQueryResultPackage>> {
        let firmware = format!("{}.{}", major, minor);
        let language_id = DbLanguage::get_language_id(conn, &lang);
        let architecture_id = DbArchitecture::get_architecute_id(conn, &arch)?; // todo return 404

        let query = sql_query(
            r#"
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
            "#,
        );

        let packages = query
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(language_id)
            .bind::<Text, _>(firmware)
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(architecture_id)
            .bind::<diesel::mysql::types::Unsigned<BigInt>, _>(build)
            .bind::<Bool, _>(beta)
            .load::<DBQueryResultPackage>(conn)
            .context("Error loading packages from DB")?;
        Ok(packages)
        // println!("{:?}", diesel::debug_query::<diesel::mysql::Mysql, _>(&q));
        // let s = String::new();
        // let os = Some(String::new());
        // let mut v = Vec::new();
        // let ob = Some(false);
        // v.push(DBQueryResultPackage{package_id:1,version_id:1,beta:false,conflictpkgs:None,deppkgs:None,changelog:os.clone(),desc:os.clone(),distributor:os.clone(),distributor_url:os.clone(),dname:os.clone(),link:os.clone(),maintainer:os.clone(),maintainer_url:os.clone(),package:s.clone(),qinst:ob,qstart:ob,qupgrade:ob,upstream_version:s.clone(),revision:1,md5:s.clone(),size:300});
        // v
    }
}

#[derive(Serialize, QueryableByName, Debug, Clone)]
pub struct DBQueryResultPackage {
    #[sql_type = "diesel::mysql::types::Unsigned<BigInt>"]
    pub package_id: u64,
    #[sql_type = "diesel::mysql::types::Unsigned<BigInt>"]
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
