use crate::schema::*;
use crate::models::DbVersion;
use crate::Connection;
use crate::{DbId, URL};
use diesel::prelude::*;

#[cfg(feature = "postgres")]
use crate::models::IconSizeEnum;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug)]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[table_name = "icon"]
pub struct DbIcon {
    pub id: DbId,
    pub version_id: DbId,
    #[cfg(feature = "postgres")]
    pub size: IconSizeEnum,
    #[cfg(feature = "mysql")]
    pub size: i32,
    #[cfg(feature = "sqlite")]
    pub size: i32,
    pub path: String,
}

impl DbIcon {
    pub fn from_version(version_id: DbId, conn: &Connection) -> Vec<Self> {
        icon::table
            .filter(icon::version_id.eq(version_id))
            .load::<Self>(conn)
            .expect("Error loading icons")
    }

    #[cfg(any(feature = "mysql", feature = "sqlite"))]
    pub fn retina_from_version(version_id: DbId, conn: &Connection) -> Vec<Self> {
        #[cfg(any(feature = "mysql", feature = "sqlite"))]
        let retina_img_size:i32 = 256;
        #[cfg(feature = "postgres")]
        let retina_img_size = IconSizeEnum::Icon256;

        icon::table
            .filter(icon::version_id.eq(version_id)).into_boxed()
            .filter(icon::size.gt(retina_img_size))
            .load::<Self>(conn)
            .expect("Error loading retina icons")
    }
    pub fn full_path(&self, package: &str) -> String {
        format!("{}/{}/{}", URL.to_string(), package, &self.path)
    }
    pub fn paths(icons: Vec<Self>, package: String) -> Vec<String> {
        icons.iter().map(|icon| icon.full_path(&package)).collect::<Vec<_>>()
    }
}
