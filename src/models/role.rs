use crate::schema::*;
use crate::Db64;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "role"]
pub struct DbRole {
    pub id: Db64,
    pub name: String,
    pub description: String,
}
