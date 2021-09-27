use crate::models::BuildArchitecture;
use crate::models::DbArchitecture;
use crate::models::DbFirmware;
use crate::models::DbVersion;
use crate::schema::*;
use crate::Connection;
use crate::Dbu32;
use crate::DbId;
use chrono::NaiveDateTime;
use diesel::dsl;
use diesel::prelude::*;
use diesel::QueryDsl;

#[derive(Serialize, Deserialize, Queryable, Associations, Identifiable, Debug, Clone)]
#[belongs_to(DbVersion, foreign_key = "version_id")]
#[belongs_to(DbFirmware, foreign_key = "firmware_id")]
#[table_name = "build"]
pub struct DbBuild {
    pub id: DbId,
    pub version_id: DbId,
    pub firmware_id: DbId,
    pub publisher_user_id: Option<DbId>,
    pub checksum: Option<String>,
    pub extract_size: Option<i32>,
    pub path: String,
    pub md5: Option<String>,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "build"]
pub struct BuildTmp {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Dbu32,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

#[derive(Serialize, Deserialize, Associations, Identifiable, Queryable, Debug, Clone)]
#[table_name = "build"]
pub struct Build {
    pub id: DbId,
    pub package: String,
    pub upstream_version: String,
    pub revision: Dbu32,
    pub architectures: Vec<String>,
    pub firmware: String,
    pub publisher: String,
    pub insert_date: NaiveDateTime,
    pub active: Option<bool>,
}

impl DbBuild {
    pub fn create_build(conn: &Connection) -> QueryResult<DbBuild>{
        // firmware
        let fw_build = 41890;
        let fw_version = "7.0";

        //package
        let package_name = "Jellyfin";

        // version
        let revision = 12;
        let upstream_version = "1.2.3";
        let changelog = "";
        let report_url = "";
        let distributor = "";
        let distributor_url = "";
        let maintainer = "";
        let maintainer_url = "";
        let dependencies = "";
        let conf_dependencies = "";
        let conflicts = "";
        let conf_conflicts = "";
        let install_wizard = false;
        let upgrade_wizard = false;
        let startable = false;
        let license = "";

        // build
        // let publisher_user_id = 152;// from api key
        let publisher_user_id = 0;// from api key
        let checksum = "";
        let md5 = "";
        let extract_size = 0;
        let path = ".spk";

        //////
        conn.build_transaction().read_write().run(|| {

            let firmware_id = firmware::table
                .filter(firmware::build.eq(fw_build))
                .filter(firmware::version.eq(fw_version))
                .select(firmware::id).first::<DbId>(conn)?;

            // package create if not available?
            let package_id = package::table
                .filter(package::name.eq(package_name))
                .select(package::id).first::<DbId>(conn)?;
            // let new_package = (package::name.eq(package_name), package::insert_date.eq(dsl::noq));
            // let package = diesel::insert_into(package::table)
            //     .values(&new_package)
            //     .get_result::<DbPackage>(conn)?;

            let t_version_id = version::table
                .filter(version::package_id.eq(package_id))
                .filter(version::ver.eq(revision))
                // .filter(version::upstream_version.eq(upstream_version))  // strict comparison
                .select(version::id).first::<DbId>(conn).optional()?;

           let version_id = match t_version_id {
                Some(id) => id,
                None => { // create a new version if one doesn't exist
                    let new_version =  (
                        version::package_id.eq(package_id),
                        version::ver.eq(revision),
                        version::upstream_version.eq(upstream_version),
                        version::changelog.eq(changelog),
                        version::report_url.eq(report_url),
                        version::distributor.eq(distributor),
                        version::distributor_url.eq(distributor_url),
                        version::maintainer.eq(maintainer),
                        version::maintainer_url.eq(maintainer_url),
                        version::dependencies.eq(dependencies),
                        version::conf_dependencies.eq(conf_dependencies),
                        version::conflicts.eq(conflicts),
                        version::conf_conflicts.eq(conf_conflicts),
                        version::install_wizard.eq(install_wizard),
                        version::upgrade_wizard.eq(upgrade_wizard),
                        version::startable.eq(startable),
                        version::license.eq(license),
                        version::insert_date.eq(dsl::now),
                    );

                    diesel::insert_into(version::table)
                        .values(&new_version)
                        .returning(version::id)
                        .get_result::<DbId>(conn)?
                    }
            };

            let new_build = (
                build::version_id.eq(version_id),
                build::firmware_id.eq(firmware_id),
                build::publisher_user_id.eq(publisher_user_id),
                build::checksum.eq(checksum),
                build::extract_size.eq(extract_size),
                build::path.eq(path),
                build::md5.eq(md5),
                build::insert_date.eq(dsl::now),
                build::active.eq(false),
            );

            let build = diesel::insert_into(build::table)
                .values(&new_build)
                .get_result::<DbBuild>(conn)?;

            Ok(build) // return id
        })
    }

    fn b_create_build(b: BuildTmp, architectures: Vec<String>) -> Build {
        Build {
            id: b.id,
            package: b.package.clone(),
            upstream_version: b.upstream_version.clone(),
            revision: b.revision,
            architectures,
            firmware: b.firmware.clone(),
            publisher: b.publisher.clone(),
            insert_date: b.insert_date,
            active: b.active,
        }
    }

    pub fn find_all(conn: &Connection, limit: i64, offset: i64) -> QueryResult<Vec<Build>> {
        // https://github.com/ChristophWurst/diesel_many_to_many/
        // https://www.reddit.com/r/rust/comments/frkta2/manytomany_relationships_in_diesel_does_anybody/
        // https://stackoverflow.com/questions/52279553/what-is-the-standard-pattern-to-relate-three-tables-many-to-many-relation-with
        // https://docs.diesel.rs/1.4.x/diesel/query_dsl/trait.BelongingToDsl.html
        // https://docs.diesel.rs/1.4.x/diesel/associations/trait.GroupedBy.html
        // https://docs.diesel.rs/1.4.x/diesel/associations/index.html
        let builds_tmp = build::table
            .limit(limit)
            .offset(offset)
            .inner_join(version::table.inner_join(package::table))
            .inner_join(firmware::table)
            .inner_join(user::table)
            .select((
                build::id,
                package::name,
                version::upstream_version,
                version::ver,
                firmware::version,
                user::username,
                build::insert_date,
                build::active,
            ))
            .load::<BuildTmp>(conn)?;

        let builds_architectures = BuildArchitecture::belonging_to(&builds_tmp)
            .inner_join(architecture::table)
            .load::<(BuildArchitecture, DbArchitecture)>(conn)?
            .grouped_by(&builds_tmp);

        // Reduce the database result to match Build struct
        let builds = builds_tmp
            .into_iter()
            .zip(builds_architectures)
            .map(|(b, ba_a)| {
                // move data from BuildTmp to Build struct
                DbBuild::b_create_build(
                    b,
                    ba_a.into_iter()
                        // drop BuildArchitecture and only get Architecture.code
                        .map(|(_, a)| a.code)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        Ok(builds)
    }

    pub fn delete_build(conn: &Connection, id: i32) -> QueryResult<usize> {
        let result = diesel::delete(build::table.filter(build::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
