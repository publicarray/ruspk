use crate::models::DbVersion;
use crate::models::DbFirmware;
use crate::schema::*;
use crate::DbId;
use chrono::NaiveDateTime;

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
    pub exec_size: i32,
    pub path: String,
    pub md5: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}
