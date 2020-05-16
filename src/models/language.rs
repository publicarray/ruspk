use crate::schema::*;
use crate::Connection;
use crate::Db64;
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "language"]
pub struct DbLanguage {
    pub id: Db64,
    pub code: String,
    pub name: String,
}

impl DbLanguage {
    pub fn get_language_id(conn: &Connection, lang: &String) -> Db64 {
        let language_id_fallback_eng: Db64 = 1;
        language::table
            .filter(language::code.eq(lang))
            .select(language::id)
            .first::<Db64>(conn)
            .unwrap_or(language_id_fallback_eng)
    }
}
