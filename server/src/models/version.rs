use crate::models::DbPackage;
use crate::schema::*;
use crate::Connection;
use crate::{Db32, DbId};
use chrono::NaiveDateTime;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[table_name = "version"]
pub struct DbVersion {
    pub id: DbId,
    pub package_id: DbId,
    pub ver: Db32,
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

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Version {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Db32,
    // beta
    pub insert_date: NaiveDateTime,
    // all active
    pub install_wizard: Option<bool>,
    pub upgrade_wizard: Option<bool>,
    pub startable: Option<bool>,
}

impl DbVersion {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<Version>> {
        version::table
            .limit(limit)
            .offset(offset)
            .inner_join(package::table)
            .select((
                version::id,
                package::name,
                version::upstream_version,
                version::ver,
                version::insert_date,
                version::install_wizard,
                version::upgrade_wizard,
                version::startable,
            ))
            .load::<Version>(conn)
    }
}
