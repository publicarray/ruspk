use crate::schema::*;
use crate::Db64;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[table_name = "firmware"]
pub struct DbFirmware {
    pub id: Db64,
    pub version: String,
    pub build: Db64,
}
