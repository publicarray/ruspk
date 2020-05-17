use crate::models::DbLanguage;
use crate::schema::*;
use crate::DbId;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbLanguage, foreign_key = "language_id")]
#[primary_key(language_id, version_id)]
#[table_name = "description"]
pub struct DbDescription {
    pub version_id: DbId,
    pub language_id: DbId,
    pub desc: String,
}
