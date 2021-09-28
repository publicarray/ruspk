use crate::models::BuildArchitecture;
use crate::models::DbArchitecture;
use crate::models::DbFirmware;
use crate::models::DbVersion;
use crate::schema::*;
use crate::Connection;
use crate::DbId;
use crate::Dbu32;
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
    pub md5: Option<String>, // should be removed
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Info {
    pub package: String,
    pub version: String,    // "1.2.3-0001"
    pub os_min_ver: String, // X.Y-Z "7.0-40000"
    pub description: String,
    pub arch: String, //space separated list "x86_64 alpine"
    pub maintainer: String,

    pub displayname: Option<String>,
    pub displayname_fre: Option<String>,
    pub description_fre: Option<String>,
    pub maintainer_url: Option<String>,
    pub distributor: Option<String>,
    pub distributor_url: Option<String>,
    pub support_url: Option<String>,
    pub support_center: Option<bool>,
    pub model: Option<String>, //space separated list "synology_bromolow_3612xs synology_cedarview_rs812rp+"
    pub exclude_arch: Option<String>, //space separated list "bromolow cedarview"
    pub checksum: Option<String>,
    pub adminport: Option<String>,        // 0~65536
    pub adminurl: Option<String>,         // "/web"
    pub adminprotocol: Option<String>,    // "http"
    pub dsmuidir: Option<String>,         // "ui"
    pub dsmappname: Option<String>,       // "SYNO.SDS.PhotoStation"
    pub dsmapppage: Option<String>,       // "SYNO.SDS.AdminCenter.Application"
    pub dsmapplaunchname: Option<String>, // "SYNO.SDS.AdminCenter.Application"
    pub checkport: Option<bool>,
    pub startable: Option<bool>,
    pub ctl_stop: Option<bool>,
    pub ctl_uninstall: Option<bool>,
    pub precheckstartstop: Option<bool>,
    pub helpurl: Option<String>,
    pub beta: Option<bool>,
    pub report_url: Option<String>,
    pub install_reboot: Option<bool>,
    pub install_dep_packages: Option<String>,      // "packageA>2.2.2:packageB"
    pub install_conflict_packages: Option<String>, // "packageA>2.2.2:packageB"
    pub install_break_packages: Option<String>,    // "packageA>2.2.2:packageB"
    pub install_replace_packages: Option<String>,  // "packageA>2.2.2:packageB"
    pub install_dep_services: Option<String>,      // "apache-web ssh"
    pub start_dep_services: Option<String>,        // "apache-web ssh"
    pub extractsize: Option<String>,               // usize "253796"
    pub support_conf_folder: Option<bool>,
    pub install_type: Option<String>, // "system"
    pub silent_install: Option<bool>,
    pub silent_upgrade: Option<bool>,
    pub silent_uninstall: Option<bool>,
    pub auto_upgrade_from: Option<String>, // "2.0"
    pub offline_install: Option<bool>,     // disable listing the package on the server, but allow manual install
    pub thirdparty: Option<bool>,
    pub os_max_ver: Option<String>, // X.Y-Z "7.0-40000"
    pub support_move: Option<bool>,
    pub exclude_model: Option<String>, // "synology_cedarview_713+ synology_kvmx64_virtualdsm"
    pub use_deprecated_replace_mechanism: Option<bool>,
    pub install_on_cold_storage: Option<bool>,

    pub changelog: Option<String>, // SynoCommunity only
}

impl DbBuild {
    pub fn create_build(conn: &Connection, info: Info, install_wizard: bool, uninstall_wizard: bool, upgrade_wizard: bool) -> QueryResult<DbBuild> {
        let info_clone = info.clone();
        let pkg_ver: Vec<&str> = info_clone.version.split("-").collect();
        let fw_min_ver: Vec<&str> = info_clone.os_min_ver.split("-").collect();
        let fw_max_ver: Vec<&str> = info_clone.os_max_ver.unwrap_or_default().split("-").collect();
        let architectures: Vec<&str> = info_clone.arch.split(" ").collect();
        // firmware
        let fw_build: i32 = fw_min_ver[1].parse().unwrap(); // todo change data type to usize
        let fw_version = fw_min_ver[0];

        // version
        let upstream_version = pkg_ver[0];
        let revision: Dbu32 = pkg_ver[1].parse().unwrap();

        let conf_dependencies = "";
        let conflicts = "";
        let conf_conflicts = "";

        // build
        let publisher_user_id = 152; // fixme from api key

        let path = format!(
            "{}/{}/{}.v{}.f{}[{}].spk",
            info.package,
            revision,
            info.package,
            revision,
            fw_build,
            architectures.join("-")
        );
        debug!("{}", path);

        let mut extractsize: Option<i32> = None;
        let t_extractsize = info_clone
            .extractsize
            .unwrap_or_default()
            .parse::<i32>()
            .unwrap_or_default();
        if t_extractsize > 0 {
            extractsize = Some(t_extractsize);
        }

        //////
        conn.build_transaction().read_write().run(|| {
            let firmware_id = firmware::table
                .filter(firmware::build.eq(fw_build))
                .filter(firmware::version.eq(fw_version))
                .select(firmware::id)
                .first::<DbId>(conn)?;

            // package create if not available?
            let package_id = package::table
                .filter(package::name.eq(info.package))
                .select(package::id)
                .first::<DbId>(conn)?;

            let t_version_id = version::table
                .filter(version::package_id.eq(package_id))
                .filter(version::ver.eq(revision))
                // .filter(version::upstream_version.eq(upstream_version))  // strict comparison
                .select(version::id)
                .first::<DbId>(conn)
                .optional()?;

            let version_id = match t_version_id {
                Some(id) => id,
                None => {
                    // create a new version if one doesn't exist
                    let new_version = (
                        version::package_id.eq(package_id),
                        version::ver.eq(revision),
                        version::upstream_version.eq(upstream_version),
                        version::changelog.eq(info.changelog),
                        version::report_url.eq(info.report_url),
                        version::distributor.eq(info.distributor),
                        version::distributor_url.eq(info.distributor_url),
                        version::maintainer.eq(info.maintainer),
                        version::maintainer_url.eq(info.maintainer_url),
                        version::dependencies.eq(info.install_dep_packages),
                        version::conf_dependencies.eq(info.install_conflict_packages),
                        // version::conflicts.eq(conflicts),
                        // version::conf_conflicts.eq(conf_conflicts),
                        version::install_wizard.eq(install_wizard),
                        // version::uninstall_wizard.eq(uninstall_wizard),
                        version::upgrade_wizard.eq(upgrade_wizard),
                        version::startable.eq(info.startable),
                        // version::license.eq(license),
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
                build::checksum.eq(info.checksum),
                build::extract_size.eq(extractsize),
                build::path.eq(path.clone()),
                // build::md5.eq(md5),
                build::insert_date.eq(dsl::now),
                build::active.eq(false),
            );

            // stop duplicate builds where the path is yet set to UNIQUE in the database.
            let exists: i64 = build::table
                .filter(build::path.eq(path.clone()))
                .count()
                .get_result(conn)?;
            if exists > 0 {
                // fixme: do proper error handling
                debug!("The file {} already exists in the database", path);
                return Err(diesel::result::Error::NotFound);
            }

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
            .order(build::id.desc())
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

    pub fn delete_build(conn: &Connection, id: DbId) -> QueryResult<usize> {
        let result = diesel::delete(build::table.filter(build::id.eq(id))).execute(conn)?;
        Ok(result)
    }
}
