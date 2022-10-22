use crate::schema::*;
use crate::DbId;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = service)]
pub struct DbService {
    pub id: DbId,
    pub code: String,
}
