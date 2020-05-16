use crate::schema::*;
use crate::Connection;
use crate::Db64;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "screenshot"]
pub struct DbScreenshot {
    pub id: Db64,
    pub package_id: Db64,
    pub path: String,
}

impl DbScreenshot {
    pub fn from_package(package_id: Db64, conn: &Connection) -> Vec<DbScreenshot> {
        screenshot::table
            .filter(screenshot::package_id.eq(package_id))
            .load::<Self>(conn)
            .expect("Error loading screenshots")
    }
}
