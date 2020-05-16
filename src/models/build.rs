use crate::models::DbFirmware;
use crate::models::DbPackage;
use crate::schema::*;
use crate::Db64;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[belongs_to(DbFirmware, foreign_key = "firmware_id")]
#[table_name = "build"]
pub struct DbBuild {
    pub id: Db64,
    pub package_id: Db64,
    pub firmware_id: Db64,
    pub publisher_user_id: Option<Db64>,
    pub checksum: Option<String>,
    pub exec_size: i32,
    pub path: String,
    pub md5: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}
