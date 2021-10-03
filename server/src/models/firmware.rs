use crate::{schema::*, utils, Connection, DbId};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: DbId,
    pub version: String,
    pub build: i32,
}

impl DbFirmware {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64, search_term: String) -> QueryResult<Vec<DbFirmware>> {
        firmware::table
            .order(firmware::build.desc())
            .filter(firmware::version.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .load::<DbFirmware>(conn)
    }

    pub fn create(conn: &Connection, version: String, build: i32) -> QueryResult<DbFirmware> {
        let firmware = diesel::insert_into(firmware::table)
            .values(&(firmware::version.eq(version), firmware::build.eq(build)))
            .get_result::<DbFirmware>(conn)?;
        Ok(firmware)
    }

    pub fn delete(conn: &Connection, id: DbId) -> QueryResult<usize> {
        let result = diesel::delete(firmware::table.filter(firmware::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
