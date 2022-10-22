use crate::models::DbLanguage;
use crate::models::DbVersion;
use crate::schema::*;
use crate::DbId;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[diesel(belongs_to(DbLanguage, foreign_key = language_id))]
#[diesel(belongs_to(DbVersion, foreign_key = version_id))]
#[diesel(primary_key(language_id, version_id))]
#[diesel(table_name = displayname)]
pub struct DbDisplayName {
    pub version_id: DbId,
    pub language_id: DbId,
    pub name: String,
}
