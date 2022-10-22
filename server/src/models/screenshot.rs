use crate::Connection;
use crate::DbId;
use crate::{schema::*, utils};
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = screenshot)]
pub struct DbScreenshot {
    pub id: DbId,
    pub package_id: DbId,
    pub path: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Screenshot {
    pub id: DbId,
    pub package: String,
    pub path: String,
}

impl DbScreenshot {
    pub fn find_all(conn: &mut Connection, limit: i64, offset: i64, search_term: String) -> QueryResult<Vec<Screenshot>> {
        screenshot::table
            .order(screenshot::id.desc())
            .filter(package::name.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .inner_join(package::table)
            .select((screenshot::id, package::name, screenshot::path))
            .load::<Screenshot>(conn)
    }

    pub fn from_package(package_id: DbId, conn: &mut Connection) -> Vec<DbScreenshot> {
        screenshot::table
            .filter(screenshot::package_id.eq(package_id))
            .load::<Self>(conn)
            .expect("Error loading screenshots")
    }

    pub fn delete(conn: &mut Connection, id: DbId) -> QueryResult<usize> {
        // todo remove file
        let result = diesel::delete(screenshot::table.filter(screenshot::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
