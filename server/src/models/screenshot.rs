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

impl DbScreenshot {
    pub fn from_package(package_id: DbId, conn: &Connection) -> Vec<DbScreenshot> {
        screenshot::table
            .filter(screenshot::package_id.eq(package_id))
            .load::<Self>(conn)
            .expect("Error loading screenshots")
    }
}
