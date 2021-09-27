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

    pub fn create_firmware(conn: &Connection, vers_str: String, build: DbId) -> QueryResult<DbFirmware> {
        let firmware = diesel::insert_into(firmware::table)
            .values(&(firmware::version.eq(vers_str), firmware::build.eq(build)))
            .get_result::<DbFirmware>(conn)?;
        Ok(firmware)
    }

    pub fn delete_firmware(conn: &Connection, id: DbId) -> QueryResult<usize> {
        let result = diesel::delete(firmware::table.filter(firmware::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
