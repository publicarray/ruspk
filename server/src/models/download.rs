use crate::models::DbArchitecture;
use crate::models::DbBuild;
use crate::schema::*;
use crate::{Db64, DbId};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[diesel(belongs_to(DbBuild, foreign_key = build_id))]
#[diesel(belongs_to(DbArchitecture, foreign_key = architecture_id))]
#[diesel(table_name = download)]
pub struct DbDownload {
    pub id: DbId,
    pub build_id: DbId,
    pub architecture_id: DbId,
    pub firmware_build: Db64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub date: Option<NaiveDateTime>,
}
