use crate::schema::*;
use crate::Db64;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "service"]
pub struct DbService {
    pub id: Db64,
    pub code: String,
}
