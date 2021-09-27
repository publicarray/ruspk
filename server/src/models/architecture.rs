use crate::schema::*;
use crate::Connection;
use crate::DbId;
use anyhow::{Context, Result};
use diesel::prelude::*;
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "architecture"]
pub struct DbArchitecture {
    pub id: DbId,
    pub code: String,
}

#[derive(Serialize, Deserialize, Insertable, Clone)]
#[table_name = "architecture"]
pub struct NewArchitecture {
    pub code: String,
}


impl DbArchitecture {
    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<DbArchitecture>> {
        architecture::table
            .limit(limit)
            .offset(offset)
            // .order(id.asc())
            .load::<DbArchitecture>(conn)
    }

    pub fn find_by_id(i: DbId, conn: &Connection) -> QueryResult<DbArchitecture> {
        architecture::table.find(i).get_result::<DbArchitecture>(conn)
    }

    pub fn get_architecture_id(conn: &Connection, arch: &str) -> Result<DbId> {
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

    pub fn new_architecture(conn: &Connection, code: String) -> QueryResult<DbId> {
        let t_new_arch = architecture::code.eq(code);

        let new = diesel::insert_into(architecture::table)
            .values(&t_new_arch)
            .returning(architecture::id)
            .get_results(conn)?;
        Ok(new[0])
    }

    pub fn delete_architecture(conn: &Connection, id: i32) -> QueryResult<usize> {
        let result = diesel::delete(architecture::table.filter(architecture::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
