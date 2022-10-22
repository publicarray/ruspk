use crate::models::DbLanguage;
use crate::schema::*;
use crate::DbId;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[diesel(belongs_to(DbLanguage, foreign_key = language_id))]
#[diesel(primary_key(language_id, version_id))]
#[diesel(table_name = description)]
pub struct DbDescription {
    pub version_id: DbId,
    pub language_id: DbId,
    pub desc: String,
}
