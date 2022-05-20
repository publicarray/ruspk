use crate::utils;
use crate::AppData;
use crate::DbId;
use crate::CACHE_TTL;
use crate::{models::*, CacheValue};
use actix_web::{delete, error, error::BlockingError, get, post, web, Error, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;

/// retrieve all packages
#[get("/api/package")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());

    let now = Instant::now();
    let cache_r = &data.cache_r;
    let key = format!("{}{}{}", &limit, &offset, &q);

    // return from cache
    if let Some(value) = cache_r.get_one(&key) {
        trace!("HIT {}ms", now.elapsed().as_millis());
        debug!("cache age {}", value.insert_time.elapsed().as_secs());
        if value.insert_time.elapsed().as_secs() < utils::str_to_u64(&CACHE_TTL) {
            return Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(&*value.http_response));
        }
    }

    // prepare response (cache miss)
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbPackage::find_all(&conn, limit, offset, q)).await;
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
            error!("{}", err);
            // Err(HttpResponse::NotFound().finish())
            Err(error::ErrorInternalServerError(err))
            // match err {
            //     BlockingError::Error(err) => match err {
            //         diesel::result::Error::NotFound => {
            //             debug!("{}", err);
            //             Err(HttpResponse::NotFound().finish())
            //         }
            //         _ => Err(error::ErrorInternalServerError(e)),
            //     },
            //     BlockingError::Canceled => Err(error::ErrorInternalServerError(e)),
            // }
        }
    }
}

/// get package by slug
#[get("/api/package/{name}")]
pub async fn get(path: web::Path<String>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let package_name = path.into_inner();
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbPackage::find(&conn, package_name))
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

#[derive(Deserialize, Clone)]
pub struct PostPackage {
    name: String,
    author_id: Option<DbId>,
}

/// create a new package
#[post("/package")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN", "DEVELOPER")]
pub async fn post(post_data: web::Json<PostPackage>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbPackage::create_package(&conn, post_data.author_id, post_data.name.clone()))
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

#[delete("/package")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN", "DEVELOPER")]
pub async fn delete(post_data: web::Json<utils::IdType>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbPackage::delete(&conn, post_data.id))
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

#[delete("/package/{id}")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN", "DEVELOPER")]
pub async fn delete_id(path: web::Path<i32>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbPackage::delete(&conn, id))
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
