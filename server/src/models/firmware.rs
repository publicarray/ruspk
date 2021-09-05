use crate::{schema::*, Connection, DbId};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: DbId,
    pub version: String,
    pub build: i32,
}

impl DbFirmware {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<DbFirmware>> {
        firmware::table.limit(limit).offset(offset).load::<DbFirmware>(conn)
    }
}
