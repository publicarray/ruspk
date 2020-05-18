use crate::schema::*;
use crate::{Db64, DbId};

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: DbId,
    pub version: String,
    pub build: Db64,
}
