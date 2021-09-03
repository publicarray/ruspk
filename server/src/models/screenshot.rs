use crate::schema::*;
use crate::Connection;
use crate::DbId;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "screenshot"]
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
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Screenshot>> {
        screenshot::table
            .limit(20)
            .offset(0)
            .inner_join(package::table)
            .select((screenshot::id, package::name, screenshot::path))
            .load::<Screenshot>(conn)
    }

    pub fn from_package(package_id: DbId, conn: &Connection) -> Vec<DbScreenshot> {
        screenshot::table
            .filter(screenshot::package_id.eq(package_id))
            .load::<Self>(conn)
            .expect("Error loading screenshots")
    }
}
