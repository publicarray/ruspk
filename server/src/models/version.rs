use crate::schema::*;
use crate::Connection;
use crate::{models::DbPackage, utils};
use crate::{DbId, Dbu32};
use chrono::NaiveDateTime;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[table_name = "version"]
pub struct DbVersion {
    pub id: DbId,
    pub package_id: DbId,
    pub ver: Dbu32, // revision
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
    pub install_dep_packages: Option<String>,
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
    pub displayname: String,
    pub description: String,
    pub upstream_version: String,
    pub revision: Dbu32,
    pub changelog: Option<String>,
    pub report_url: Option<String>,
    pub insert_date: NaiveDateTime,
    // all active
    pub install_wizard: Option<bool>,
    pub upgrade_wizard: Option<bool>,
    pub startable: Option<bool>,
}

impl DbVersion {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64, search_term: String) -> QueryResult<Vec<Version>> {
        version::table
            .order(version::id.desc())
            .filter(displayname::name.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .inner_join(
                displayname::table.on(displayname::version_id
                    .eq(version::id)
                    .and(displayname::language_id.eq(1))),
            )
            .inner_join(
                description::table.on(description::version_id
                    .eq(version::id)
                    .and(description::language_id.eq(1))),
            )
            .inner_join(package::table)
            .select((
                version::id,
                package::name,
                displayname::name,
                description::desc,
                version::upstream_version,
                version::ver, // revision
                version::changelog,
                version::report_url, // beta //fix me: convert to bool
                version::insert_date,
                version::install_wizard,
                version::upgrade_wizard,
                version::startable,
            ))
            .load::<Version>(conn)
    }

    pub fn delete(conn: &Connection, id: DbId) -> QueryResult<usize> {
        conn.build_transaction().read_write().run(|| {
            let build_ids = build::table
                .filter(build::version_id.eq(id))
                .select(build::id)
                .load::<DbId>(conn)?;
            for build_id in build_ids {
                diesel::delete(build_architecture::table.filter(build_architecture::build_id.eq(build_id)))
                    .execute(conn)?;
            }
            let builds = diesel::delete(build::table.filter(build::version_id.eq(id))).execute(conn)?;
            let versions = diesel::delete(version::table.filter(version::id.eq(id))).execute(conn)?;
            Ok(builds + versions) // number of rows effected
        })
    }
}
