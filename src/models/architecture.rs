use crate::schema::*;
use diesel::prelude::*;
use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[table_name = "architecture"]
pub struct DbArchitecture {
    pub id: u64,
    pub code: String,
}

impl DbArchitecture {
    pub fn get_architecute_id(conn: &MysqlConnection, arch: &String) -> Result<u64> {

        let architecture_id = architecture::table
            .filter(architecture::code.eq(arch))
            .select(architecture::id)
            .first::<u64>(conn)
            .context("Error loading architecture from DB")?; // todo return 404
        Ok(architecture_id)
    }

}
