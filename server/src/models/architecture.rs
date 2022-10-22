use crate::Connection;
use crate::DbId;
use crate::{schema::*, utils};
use anyhow::{Context, Result};
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = architecture)]
pub struct DbArchitecture {
    pub id: DbId,
    pub code: String,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[diesel(table_name = architecture)]
pub struct NewArchitecture {
    pub code: String,
}

impl DbArchitecture {
    pub fn find_all(
        conn: &mut Connection,
        limit: i64,
        offset: i64,
        search_term: String,
    ) -> QueryResult<Vec<DbArchitecture>> {
        architecture::table
            .order(architecture::id.desc())
            .filter(architecture::code.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .load::<DbArchitecture>(conn)
    }

    pub fn find_by_id(i: DbId, conn: &mut Connection) -> QueryResult<DbArchitecture> {
        architecture::table.find(i).get_result::<DbArchitecture>(conn)
    }

    pub fn get_architecture_id(conn: &mut Connection, arch: &str) -> Result<DbId> {
        let arch = match arch {
            "88f6281" => "88f628x",
            "88f6282" => "88f628x",
            _ => arch,
        };

        // to_syno = {"88f628x": "88f6281"}

        let architecture_id = architecture::table
            .filter(architecture::code.eq(arch))
            .select(architecture::id)
            .first::<DbId>(conn)
            .context("Error loading architecture from DB")?; // todo return 404
        Ok(architecture_id)
    }

    pub fn create(conn: &mut Connection, code: String) -> QueryResult<DbArchitecture> {
        let arch = diesel::insert_into(architecture::table)
            .values(&architecture::code.eq(code))
            .get_result::<DbArchitecture>(conn)?;
        Ok(arch)
    }

    pub fn delete(conn: &mut Connection, id: DbId) -> QueryResult<usize> {
        // todo remove everything else linked to the architecture
        let result = diesel::delete(architecture::table.filter(architecture::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
