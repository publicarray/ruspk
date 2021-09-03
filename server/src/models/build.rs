use crate::Db32;
use crate::models::DbFirmware;
use crate::models::DbVersion;
use crate::schema::*;
use crate::DbId;
use chrono::NaiveDateTime;
use diesel::QueryDsl;
use crate::Connection;
use diesel::prelude::*;

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


#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]

pub struct Build1 {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Db32,
    // pub architectures: String,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Build {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Db32,
    pub architectures: Vec<String>,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

impl DbBuild {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Build>> {
        let builds_by_id = build::table.select(build::id).load::<DbId>(conn)?;
        let mut architectures: Vec<Vec<String>> = Vec::new();
        for build_id in builds_by_id {
            let arch = build_architecture::table
                .filter(build_architecture::build_id.eq(build_id))
                .inner_join(architecture::table)
                .select(architecture::code)
                .load::<String>(conn)?;
            architectures.push(arch)
        }

        let db_builds = build::table
            .limit(20)
            .offset(0)
            .inner_join(version::table.inner_join(package::table))
            .inner_join(firmware::table)
            .inner_join(user::table)
            .inner_join(build_architecture::table.inner_join(architecture::table))
            .select((
                build::id,
                package::name,
                version::upstream_version,
                version::ver,
                // architecture::code, // as an array or vector
                firmware::version,
                user::username,
                build::insert_date,
                build::active))
            .order_by(build::id.asc())
            .load::<Build1>(conn)
            .expect("Failed to get builds from db");

        let mut builds: Vec<Build> = Vec::new();
        for (i, b) in db_builds.iter().enumerate() {
            builds.push(
                Build {
                    id: b.id,
                    package: b.package.clone(),
                    upstream_version: b.upstream_version.clone(),
                    revision: b.revision.clone(),
                    architectures: architectures[i].clone(),
                    firmware: b.firmware.clone(),
                    publisher: b.publisher.clone(),
                    insert_date: b.insert_date,
                    active: b.active,
                }
            )
        }
        return Ok(builds);
    }
}
