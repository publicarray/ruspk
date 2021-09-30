use crate::models::*;
use crate::utils;
use crate::{AppData, DbConn};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;

/// retrieve all firmware
#[get("/firmware")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset) = utils::paginate_qs(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbFirmware::find_all(&conn, limit, offset))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize, Clone)]
pub struct CreateFirmware{
    version: String,
    build: i32,
}

#[post("/firmware")]
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
