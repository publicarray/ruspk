use crate::models::*;
use actix_web::{error::BlockingError, web, Error, HttpRequest, HttpResponse, Responder};

use crate::synopackagelist::*;
use crate::{AppData, Db64, Db8, DbConn, DbId};
use anyhow::Result;
use diesel::{self, prelude::*};
use std::sync::Arc;
use std::time::Instant;

pub const KEYRING: &str = "-----BEGIN PGP PUBLIC KEY BLOCK-----\nVersion: GnuPG v1\n\nmQENBFRhPdoBCADMWckT2GJRrRcuNXuCBNp3oSC7tlQxa1EN81HhlX2Tqs7tKezC\nvgGCB69jWQmfB5BKdWcznS98bLZB4Iy2RjU8vtkI0/6AceovCytMXW0d6HE8frlf\n6gkWKylRbD3fBE+qpHOEwpV5MTEgF8UTM9cPzupY6ggm/6fSDxXqYRZQHfnAFoLE\nXSzMtdUyY0w4a1CapfVRa060XRNLvacu6+1XVksJVZbuChg3/zDhtYZuvbuXxwfA\n/sZem9OW85roUgsYE3cL8m4iexZHMIbWBO5mj7LoYgb33vF7T15yGUjWADMbBkQx\nYFBg6q48Nc81taFHRWpIIXe1s1ZTxyBQL0hjABEBAAG0NFN5bm9Db21tdW5pdHkg\nKE9mZmljaWFsKSA8Y29udGFjdEBzeW5vY29tbXVuaXR5LmNvbT6JATgEEwECACIF\nAlRhPdoCGwMGCwkIBwMCBhUIAgkKCwQWAgMBAh4BAheAAAoJENDC8YaUoLiOtJkI\nAKpGpoKrmkzSFEhSj+tbTx/EdsrQu+9Q32H51EZlM0gSn1rzjPBsg20Uh3JoK2gO\nDrWtcL6wSgd6Vp9ScwcjH/GQ6fh5/AIcXl1oW/Z31ZiqGxJmdT1EwdqYZdN+bv8K\nF4rezHKwlUAsq4jHvwnmOfjqJzn4Gpbf0diajLBNMmZY0uOe8Q0s1IqNkrtw0zWD\nimZqYTrktnpm8YFDUe1xo6qRNdqVXn5lddfrO4hPDP2/hzgZ6l0Gnl4P0ZFYAGo9\nQITV2BqBbBpMYff/yF0yxbSQgCu93J3FtMe3qK6mu2lclSDEFs+abX0NIbUOTv4l\nAus7c0ZXjYOZNLAYY+RAXsO5AQ0EVGE92gEIAJw1AdFZ1MXlU+JeCLqes8LV3Grv\nhTvTRWTd7Pi3W+qoaGkeTCfc9Jxf5PgFr0s5ZJrXD6f/JF73JSFwuHrGacSAR28/\nnPcLZPN5JYDipWmSdoa672lEeDJ+Zr2f2jtFs0CTXbyTyVSZnoDtL5j7a3BtlJ6N\nw2FaGVeqto7qfkudizEnoNcokmeAD0EpajCq2L0ZO6QxTP8q3gVoffQK6UTOublJ\nHj1T5A1ZH+hgVmjAsQ7AOh3ElRml+lkd3j0luYiuMiz8ol3GHjTQ0C5GnbWka3LH\nnrgU75kJduGtngEnmR6dBZPR47ImjsX5cQ7JWrJLSrWc907+7vcb6cAwYcUAEQEA\nAYkBHwQYAQIACQUCVGE92gIbDAAKCRDQwvGGlKC4jgEUB/9jwTxRbVGKjVyO9ZdP\nYR5seJU0R3ZUKZa5+Qv7BXPSaBS6nCHejxOd9Jg8zYafVTDdCYdvDfNrKnhhKOC9\n637WGNd/T7LfPH0fp7KHKv+QJ15LhleMpcsKVt8+32px7jepAltD6drNTATkDyST\nQz5PMrVZLkhZo2zu/I8sfj/SAd0mtoBBpRfA3Xt9AWCMqaWcSM9nmz3awzJopVY3\nUXnX9p13B4op2wyPnoW0j1GdBRv/Ky2kOYE++AczGwhbPos2fD3Ulg4aIKspgZ5f\nsvlMBaG/AAd69IVvWQYqlUvyB1v6i1Trl6Ti2sNd6eAphNAJeQGCTcU3w6ibvAq5\nyshz\n=pO8s\n-----END PGP PUBLIC KEY BLOCK-----";

pub async fn index(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Serialize, Deserialize)]
pub struct SynoRequest {
    arch: String,                           // apollolake
    build: Db64,                            // 24922
    language: String,                       // enu
    major: Db8,                             // 6
    micro: Db8,                             // 2
    minor: Db8,                             // 2
    nano: Option<Db8>,                      // 4
    package_update_channel: Option<String>, // beta/stable
    timezone: Option<String>,               // London
    unique: Option<String>,                 // synology_apollolake_418play
}

pub async fn syno(
    data: web::Data<AppData>,
    synorequest: web::Query<SynoRequest>,
) -> Result<HttpResponse, HttpResponse> {
    let now = Instant::now();

    let cache_r = &data.cache_r;

    let key = format!(
        "{}{}{}{:?}{}{}{}",
        &synorequest.language,
        &synorequest.arch,
        synorequest.build,
        &synorequest.package_update_channel,
        synorequest.major,
        synorequest.micro,
        synorequest.minor
    );

    if let Some(response_cache) = cache_r.get_one(&key) {
        trace!("HIT {}ms", now.elapsed().as_millis());
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(&*response_cache));
    } else {
        let conn = data.pool.get().expect("couldn't get db connection from pool");
        let response = web::block(move || {
            get_packages_for_device_lang(
                &conn,
                &synorequest.language,
                &synorequest.arch,
                synorequest.build,
                &synorequest.package_update_channel,
                synorequest.major,
                synorequest.micro,
                synorequest.minor,
            )
        })
        .await;
        trace!("MISS {}ms", now.elapsed().as_millis());
        match response {
            Ok(packages) => {
                let value = serde_json::to_string(&packages).unwrap();
                let cache_w_arc = Arc::clone(&data.cache_w);
                let mut cache_w = cache_w_arc.lock().unwrap();
                cache_w.insert(key, value);
                cache_w.refresh();
                Ok(HttpResponse::Ok().json(&packages))
            }
            Err(err) => {
                trace!("{}", err);
                match err {
                    BlockingError::Error(err) => match err.downcast_ref::<diesel::result::Error>().unwrap() {
                        diesel::result::Error::NotFound => {
                            debug!("{}", err);
                            Err(HttpResponse::NotFound().finish())
                        }
                        _ => Err(HttpResponse::InternalServerError().finish()),
                    },
                    BlockingError::Canceled => Err(HttpResponse::InternalServerError().finish()),
                }
            }
        }
    }
}

fn get_package(conn: &DbConn) -> Result<Vec<DbPackage>> {
    use crate::schema::package;
    let p = package::table
        .load::<DbPackage>(conn)
        .expect("Error loading package from DB");
    Ok(p)
}

pub async fn list_packages(data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || get_package(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}

fn get_version(conn: &DbConn, num: DbId) -> Result<Vec<DbVersion>> {
    use crate::schema::version::dsl::*;
    let v = version
        .filter(package_id.eq(num))
        .load(conn)
        .expect("Error loading version from DB");
    Ok(v)
}

pub async fn get_package_version(data: web::Data<AppData>, id: web::Path<DbId>) -> Result<HttpResponse, HttpResponse> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || get_version(&conn, *id)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    if response.is_empty() {
        return Err(HttpResponse::NotFound().finish());
    }
    Ok(HttpResponse::Ok().json(response))
}
