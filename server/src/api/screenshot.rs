use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_web::{delete, get, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;
use actix_web_grants::proc_macro::has_any_role;

/// retrieve all screenshots
#[get("/screenshot")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbScreenshot::find_all(&conn, limit, offset, q))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/screenshot")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete(post_data: web::Json<utils::IdType>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbScreenshot::delete(&conn, post_data.id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/screenshot/{id}")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete_id(web::Path(id): web::Path<i32>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbScreenshot::delete(&conn, id)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}
