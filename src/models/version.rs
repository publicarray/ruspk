use crate::models::DbPackage;
use crate::schema::*;
use crate::{Db32, Db64};
use chrono::NaiveDateTime;
#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbPackage, foreign_key = "package_id")]
#[table_name = "version"]
pub struct DbVersion {
    pub id: Db64,
    pub package_id: Db64,
    pub ver: Db32,
    pub upstream_version: String,
    pub changelog: Option<String>,
    pub report_url: Option<String>,
    pub distributor: Option<String>,
    pub distributor_url: Option<String>,
    pub maintainer: Option<String>,
    pub maintainer_url: Option<String>,
    pub dependencies: Option<String>,
    pub conf_dependencies: Option<String>,
    pub conflicts: Option<String>,
    pub conf_conflicts: Option<String>,
    pub install_wizard: Option<bool>,
    pub upgrade_wizard: Option<bool>,
    pub startable: Option<bool>,
    pub license: Option<String>,
    pub insert_date: NaiveDateTime,
}
