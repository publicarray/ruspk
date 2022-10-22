use crate::schema::*;
use crate::DbId;
#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = role)]
pub struct DbRole {
    pub id: DbId,
    pub name: String,
    pub description: String,
}
