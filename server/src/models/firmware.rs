use crate::{Connection, Db32, DbId, schema::*};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: DbId,
    pub version: String,
    pub build: Db32,
}

impl DbFirmware {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<DbFirmware>> {
        firmware::table
            .limit(20)
            .offset(0)
            .load::<DbFirmware>(conn)
    }
}
