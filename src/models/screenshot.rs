use crate::schema::*;
use crate::DbConn;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "screenshot"]
pub struct DbScreenshot {
    pub id: u64,
    pub package_id: u64,
    pub path: String,
}

impl DbScreenshot {
    pub fn from_package(package_id: u64, conn: &DbConn) -> Vec<DbScreenshot> {
        screenshot::table
            .filter(screenshot::package_id.eq(package_id))
            .load::<Self>(&**conn)
            .expect("Error loading screenshots")
    }
}
