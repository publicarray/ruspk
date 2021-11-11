use crate::schema::*;
use crate::DbId;
#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "role"]
pub struct DbRole {
    pub id: DbId,
    pub name: String,
    pub description: String,
}
