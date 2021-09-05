use crate::models::BuildArchitecture;
use crate::models::DbArchitecture;
use crate::models::DbFirmware;
use crate::models::DbVersion;
use crate::schema::*;
use crate::Connection;
use crate::Dbu32;
use crate::DbId;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::QueryDsl;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[belongs_to(DbFirmware, foreign_key = "firmware_id")]
#[table_name = "build"]
pub struct DbBuild {
    pub id: DbId,
    pub version_id: DbId,
    pub firmware_id: DbId,
    pub publisher_user_id: Option<DbId>,
    pub checksum: Option<String>,
    pub extract_size: Option<i32>,
    pub path: String,
    pub md5: Option<String>,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "build"]
pub struct BuildTmp {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Dbu32,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "build"]
pub struct Build {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Dbu32,
    pub architectures: Vec<String>,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

impl DbBuild {
    pub fn new_build(b: BuildTmp, architectures: Vec<String>) -> Build {
        Build {
            id: b.id,
            package: b.package.clone(),
            upstream_version: b.upstream_version.clone(),
            revision: b.revision,
            architectures,
            firmware: b.firmware.clone(),
            publisher: b.publisher.clone(),
            insert_date: b.insert_date,
            active: b.active,
        }
    }

    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<Build>> {
        // https://github.com/ChristophWurst/diesel_many_to_many/
        // https://www.reddit.com/r/rust/comments/frkta2/manytomany_relationships_in_diesel_does_anybody/
        // https://stackoverflow.com/questions/52279553/what-is-the-standard-pattern-to-relate-three-tables-many-to-many-relation-with
        // https://docs.diesel.rs/1.4.x/diesel/query_dsl/trait.BelongingToDsl.html
        // https://docs.diesel.rs/1.4.x/diesel/associations/trait.GroupedBy.html
        // https://docs.diesel.rs/1.4.x/diesel/associations/index.html
        let builds_tmp = build::table
            .limit(limit)
            .offset(offset)
            .inner_join(version::table.inner_join(package::table))
            .inner_join(firmware::table)
            .inner_join(user::table)
            .select((
                build::id,
                package::name,
                version::upstream_version,
                version::ver,
                firmware::version,
                user::username,
                build::insert_date,
                build::active,
            ))
            .load::<BuildTmp>(conn)?;

        let builds_architectures = BuildArchitecture::belonging_to(&builds_tmp)
            .inner_join(architecture::table)
            .load::<(BuildArchitecture, DbArchitecture)>(conn)?
            .grouped_by(&builds_tmp);

        // Reduce the database result to match Build struct
        let builds = builds_tmp
            .into_iter()
            .zip(builds_architectures)
            .map(|(b, ba_a)| {
                // move data from BuildTmp to Build struct
                DbBuild::new_build(
                    b,
                    ba_a.into_iter()
                        // drop BuildArchitecture and only get Architecture.code
                        .map(|(_, a)| a.code)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        Ok(builds)
    }
}
