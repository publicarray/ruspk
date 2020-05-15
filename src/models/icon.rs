use crate::models::DbVersion;
use crate::schema::*;
use crate::URL;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[table_name = "icon"]
pub struct DbIcon {
    pub id: u64,
    pub version_id: u64,
    pub size: i32,
    pub path: String,
}

impl DbIcon {
    pub fn from_version(version_id: u64, conn: &MysqlConnection) -> Vec<Self> {
        icon::table
            .filter(icon::version_id.eq(version_id))
            .load::<Self>(conn)
            .expect("Error loading icons")
    }
    pub fn retina_from_version(version_id: u64, conn: &MysqlConnection) -> Vec<Self> {
        icon::table
            .filter(icon::version_id.eq(version_id))
            .filter(icon::size.gt(256))
            .load::<Self>(conn)
            .expect("Error loading icons")
    }
    pub fn full_path(&self, package: &String) -> String {
        format!("{}/{}/{}", URL.to_string(), package, &self.path)
    }
    pub fn paths(icons: Vec<Self>, package: String) -> Vec<String> {
        icons.iter().map(|icon| icon.full_path(&package)).collect::<Vec<_>>()
    }
}
