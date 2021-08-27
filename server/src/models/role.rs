use crate::schema::*;
use crate::DbId;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "role"]
pub struct DbRole {
    pub id: DbId,
    pub name: String,
    pub description: String,
}
