use crate::models::DbLanguage;
use crate::models::DbVersion;
use crate::schema::*;
use crate::DbId;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[primary_key(language_id, version_id)]
#[table_name = "displayname"]
pub struct DbDisplayName {
    pub version_id: DbId,
    pub language_id: DbId,
    pub name: String,
}
