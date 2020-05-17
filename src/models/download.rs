use crate::models::DbArchitecture;
use crate::models::DbBuild;
use crate::schema::*;
use crate::{DbId, Db64};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbBuild, foreign_key = "build_id")]
#[belongs_to(DbArchitecture, foreign_key = "architecture_id")]
#[table_name = "download"]
pub struct DbDownload {
    pub id: DbId,
    pub build_id: DbId,
    pub architecture_id: DbId,
    pub firmware_build: Db64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub date: Option<NaiveDateTime>,
}
