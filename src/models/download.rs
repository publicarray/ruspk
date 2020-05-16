use crate::models::DbArchitecture;
use crate::models::DbBuild;
use crate::schema::*;
use crate::Db64;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[table_name = "download"]
pub struct DbDownload {
    pub id: Db64,
    pub build_id: Db64,
    pub architecture_id: Db64,
    pub firmware_build: Db64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub date: Option<NaiveDateTime>,
}
