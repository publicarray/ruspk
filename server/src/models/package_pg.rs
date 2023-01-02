use crate::schema::*;
use crate::Connection;
use crate::{models::DbArchitecture, utils};
use crate::{models::DbLanguage, DbId};
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use diesel::dsl::{self, max};
use diesel::prelude::*;
use diesel::query_builder::SqlQuery;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Bool, Integer, Nullable, Text};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug, Clone)]
#[diesel(table_name = package)]
pub struct DbPackage {
    pub id: i32,
    pub author_user_id: Option<i32>,
    pub name: String,
    pub insert_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct TmpPackage {
    pub id: i32,
    pub author: Option<String>,
    pub name: String,
    pub displayname: Option<String>,
    pub description: String,
    pub version: String,
    pub revision: i32,
    pub insert_date: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct Package {
    pub id: i32,
    pub author: Option<String>,
    pub name: String,
    pub displayname: Option<String>,
    pub description: String,
    pub version: String, // Latest version
    pub revision: i32,   // Latest revision
    pub insert_date: Option<NaiveDateTime>,
}

impl DbPackage {
    pub fn find_all(conn: &mut Connection, limit: i64, offset: i64, search_term: String) -> QueryResult<Vec<Package>> {
        let package_ids = package::table
            .order(package::name)
            .filter(package::name.ilike(utils::fuzzy_search(&search_term)))
            .limit(limit)
            .offset(offset)
            .select(package::id)
            .load::<DbId>(conn)?;
        let mut packages: Vec<Package> = vec![];

        for id in package_ids {
            let tmp_latest_revision = version::table
                .filter(version::package_id.eq(id))
                .select(max(version::ver))
                .load::<Option<i32>>(conn)?;

            // there should only be one result
            if tmp_latest_revision.len() != 1 {
                continue;
            };

            let latest_revision = match tmp_latest_revision[0] {
                Some(rev) => rev,
                _ => {
                    warn!(
                        "Package ID:{} does not have a version {:?}. ignoring.", // todo fix this for the admin view
                        id, tmp_latest_revision
                    );
                    continue;
                }
            };

            let package = package::table
                .filter(package::id.eq(id))
                .left_join(user::table)
                .inner_join(
                    version::table.on(version::package_id
                        .eq(package::id)
                        .and(version::ver.eq(latest_revision))),
                )
                .left_join(
                    displayname::table.on(displayname::version_id
                        .eq(version::id)
                        .and(displayname::language_id.eq(1))),
                )
                .inner_join(
                    description::table.on(description::version_id
                        .eq(version::id)
                        .and(description::language_id.eq(1))),
                )
                .select((
                    package::id,
                    user::username.nullable(),
                    package::name,
                    displayname::name.nullable(),
                    description::desc,
                    version::upstream_version, //nop
                    version::ver,              // nop
                    package::insert_date,
                ))
                .first::<Package>(conn)?;
            packages.push(package);
        }

        Ok(packages)
    }

    pub fn find(conn: &mut Connection, package_name: String) -> QueryResult<Package> {
        // should be version
        package::table
            .filter(package::name.eq(package_name))
            .left_join(user::table)
            .inner_join(version::table)
            .left_join(
                displayname::table.on(displayname::version_id
                    .eq(version::id)
                    .and(displayname::language_id.eq(1))),
            )
            .inner_join(
                description::table.on(description::version_id
                    .eq(version::id)
                    .and(description::language_id.eq(1))),
            )
            .select((
                package::id,
                user::username.nullable(),
                package::name,
                displayname::name.nullable(),
                description::desc,
                version::upstream_version,
                version::ver,
                package::insert_date,
            ))
            .first::<Package>(conn)
    }

    pub fn create_package(conn: &mut Connection, author_id: Option<i32>, name: String) -> QueryResult<DbPackage> {
        let new_package = (
            package::author_user_id.eq(author_id),
            package::name.eq(name),
            package::insert_date.eq(dsl::now),
        );

        let package = diesel::insert_into(package::table)
            .values(&new_package)
            .get_result::<DbPackage>(conn)?;
        Ok(package)
    }

    // todo: update or delete on table "build" violates foreign key constraint "build_architecture_build_id_fkey" on table "build_architecture"
    pub fn delete(conn: &mut Connection, id: i32) -> QueryResult<usize> {
        conn.build_transaction().read_write().run(|conn| {
            let builds = diesel::delete(build::table.filter(
                build::version_id.eq_any(version::table.filter(version::package_id.eq(id)).select(version::id)),
            ))
            .execute(conn)?;

            let versions = diesel::delete(version::table.filter(version::package_id.eq(id))).execute(conn)?;

            let packages = diesel::delete(package::table.filter(package::id.eq(id))).execute(conn)?;

            Ok(builds + versions + packages) // number of rows effected
        })
    }

    // NAS api
    pub fn get_packages(
        lang: &str,
        arch: &str,
        build: i64,
        beta: bool,
        _major: i8,
        _micro: i8,
        _minor: i8,
        conn: &mut Connection,
    ) -> Result<Vec<DBQueryResultPackage>> {
        let language_id: i32 = DbLanguage::get_language_id(conn, lang);
        let architecture_id: i32 = DbArchitecture::get_architecture_id(conn, arch)
            .context(format!("Can't find architecture in DB for {}", &arch))?; // todo return 404

        let query = sql_query(
            r#"
                SELECT
                package.id AS package_id,
                version.id AS version_id,
                (CASE WHEN version.report_url <> '' THEN true ELSE false END) AS beta,
                version.conflicts AS conflictpkgs,
                version.dependencies AS deppkgs,
                version.changelog,
                description.description AS "desc",
                version.distributor,
                version.distributor_url,
                displayname.displayname AS dname,
                build.path AS link,
                version.maintainer,
                version.maintainer_url,
                package.name AS package,
                version.install_wizard AS qinst,
                version.startable AS qstart,
                version.upgrade_wizard AS qupgrade,
                version.upstream_version,
                version.version AS revision,
                build.md5,
                build.extract_size AS size

                FROM
                (
                    (
                    package
                    INNER JOIN (
                        (
                        (
                            version
                            LEFT OUTER JOIN description ON description.version_id = version.id
                            AND description.language_id = CASE WHEN EXISTS (
                            -- language_id 1=english
                            SELECT 1
                            FROM description
                            WHERE description.language_id = $1
                            ) THEN $1 ELSE 1 END
                        )
                        LEFT OUTER JOIN displayname ON displayname.version_id = version.id
                        AND displayname.language_id = CASE WHEN EXISTS (
                            SELECT  1
                            FROM displayname
                            WHERE displayname.language_id = $1
                        ) THEN $1 ELSE 1 END
                        )
                        INNER JOIN (
                        SELECT MAX(version.version) AS version, package_id
                        FROM version
                        GROUP BY version.package_id
                        ) ver ON version.package_id = ver.package_id
                        AND version.version = ver.version
                    ) ON version.package_id = package.id
                    )
                    INNER JOIN (
                    (
                        build
                        INNER JOIN firmware ON firmware.id = build.firmware_id
                    )
                    INNER JOIN build_architecture ON build_architecture.build_id = build.id
                    -- architecture_id 1=noarch
                    AND build_architecture.architecture_id IN(1, $2)
                    ) ON build.version_id = version.id
                )
                WHERE build.active = true
                AND firmware.build <= $3
                AND ($4 OR (version.report_url = '' OR version.report_url IS NULL))
            "#,
        );
        let packages = bind_and_load(conn, query, language_id, architecture_id, build, beta)?;
        Ok(packages)
    }
}

pub fn bind_and_load(
    conn: &mut Connection,
    query: SqlQuery,
    language_id: i32,
    architecture_id: i32,
    build: i64,
    beta: bool,
) -> Result<Vec<DBQueryResultPackage>> {
    let result = query
        .bind::<Integer, _>(language_id)
        .bind::<Integer, _>(architecture_id)
        .bind::<BigInt, _>(build)
        .bind::<Bool, _>(beta)
        .load::<DBQueryResultPackage>(conn)
        .context("Error loading packages from DB")?;
    Ok(result)
}

#[derive(Serialize, QueryableByName, Debug, Clone)]
pub struct DBQueryResultPackage {
    #[diesel(sql_type = Integer)]
    pub package_id: i32,
    #[diesel(sql_type = Integer)]
    pub version_id: i32,
    #[diesel(sql_type = Bool)]
    pub beta: bool,
    #[diesel(sql_type = Nullable<Text>)]
    pub conflictpkgs: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub deppkgs: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub changelog: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub desc: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub distributor: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub distributor_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub dname: Option<String>,
    // download_count: i32,
    #[diesel(sql_type = Nullable<Text>)]
    pub link: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub maintainer: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub maintainer_url: Option<String>,
    #[diesel(sql_type = Text)]
    pub package: String,
    #[diesel(sql_type = Nullable<Bool>)]
    pub qinst: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub qstart: Option<bool>,
    #[diesel(sql_type = Nullable<Bool>)]
    pub qupgrade: Option<bool>,
    // recent_download_count: i32,
    #[diesel(sql_type = Text)]
    pub upstream_version: String,
    #[diesel(sql_type = Integer)]
    pub revision: i32,
    #[diesel(sql_type = Nullable<Text>)]
    pub md5: Option<String>,
    #[diesel(sql_type = Nullable<Integer>)]
    pub size: Option<i32>,
}
