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
pub struct Build {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Db32,
    pub architectures: String,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

impl DbBuild {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Build>> {

        build::table
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
            architecture::code, // as an array or vector
            firmware::version,
            user::username,
            build::insert_date,
            build::active))
        .order_by(build::id.asc())
        .load::<Build>(conn)
    }
}
