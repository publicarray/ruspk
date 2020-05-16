use crate::schema::*;
use crate::Db64;
use chrono::NaiveDateTime;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "user"]
pub struct DbUser {
    pub id: Db64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub api_key: Option<String>,
    pub github_access_token: Option<String>,
    pub active: bool,
    pub confirmed_at: Option<NaiveDateTime>,
}
