use crate::schema::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "role"]
pub struct DbRole {
    pub id: u64,
    pub name: String,
    pub description: String,
}
