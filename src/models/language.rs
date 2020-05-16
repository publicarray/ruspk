use crate::schema::*;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "language"]
pub struct DbLanguage {
    pub id: u64,
    pub code: String,
    pub name: String,
}

impl DbLanguage {
    pub fn get_language_id(conn: &MysqlConnection, lang: &String) -> u64 {
        let language_id_fallback_eng: u64 = 1;
        language::table
            .filter(language::code.eq(lang))
            .select(language::id)
            .first::<u64>(conn)
            .unwrap_or(language_id_fallback_eng)
    }
}
