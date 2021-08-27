use crate::schema::*;
use crate::Connection;
use crate::DbId;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "language"]
pub struct DbLanguage {
    pub id: DbId,
    pub code: String,
    pub name: String,
}

impl DbLanguage {
    pub fn get_language_id(conn: &Connection, lang: &str) -> DbId {
        let language_id_fallback_eng: DbId = 1;
        language::table
            .filter(language::code.eq(lang))
            .select(language::id)
            .first::<DbId>(conn)
            .unwrap_or(language_id_fallback_eng)
    }
}
