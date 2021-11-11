use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;
use actix_web_grants::proc_macro::{has_any_role, has_permissions};

/// retrieve all firmware
#[get("/firmware")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbFirmware::find_all(&conn, limit, offset, q))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize, Clone)]
pub struct CreateFirmware {
    version: String,
    build: i32,
}

#[post("/firmware")]
#[has_any_role("ADMIN", "PACKAGE_DEV", "DEV")]
pub async fn post(post_data: web::Json<CreateFirmware>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbFirmware::create(&conn, post_data.version.clone(), post_data.build))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
