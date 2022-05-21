use crate::models::*;
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder};

use crate::synopackagelist::*;
use crate::{AppData, CacheValue, Db64, Db8, DbConn, DbId, CACHE_TTL};
use anyhow::Result;
use diesel::{self, prelude::*};
use std::sync::Arc;
use std::time::Instant;

use crate::utils;

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

pub async fn syno(data: web::Data<AppData>, synorequest: web::Query<SynoRequest>) -> Result<HttpResponse, Error> {
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

    if let Some(value) = cache_r.get_one(&key) {
        trace!("HIT {}ms", now.elapsed().as_millis());
        debug!("cache age {}", value.insert_time.elapsed().as_secs());
        if value.insert_time.elapsed().as_secs() < utils::str_to_u64(&CACHE_TTL) {
            let response = &*value.http_response;
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(response.to_owned()));
        }
    }

    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let keyring = data.keyring.clone();
    let response = web::block(move || {
        get_packages_for_device_lang(
            &conn,
            &keyring,
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
            let packages = packages.map_err(|e| {
                debug!("{}", e);
                error::ErrorInternalServerError(e)
            })?;

            let http_response = serde_json::to_string(&packages).unwrap();
            let value = CacheValue {
                http_response: Arc::new(http_response),
                insert_time: Arc::new(Instant::now()),
            };
            let cache_w_arc = Arc::clone(&data.cache_w);
            let mut cache_w = cache_w_arc.lock().unwrap();
            if cache_r.get(&key).map(|rs| rs.len()) == None {
                cache_w.insert(key, value);
            } else {
                cache_w.update(key, value);
            }
            cache_w.refresh();
            Ok(HttpResponse::Ok().json(&packages))
        }
        Err(err) => {
            error!("{:?}", err);
            Err(error::ErrorInternalServerError(err))
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
    let response = web::block(move || get_package(&conn))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
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
    let response = web::block(move || get_version(&conn, *id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;
    if response.is_empty() {
        return Err(HttpResponse::NotFound().finish());
    }
    Ok(HttpResponse::Ok().json(response))
}
