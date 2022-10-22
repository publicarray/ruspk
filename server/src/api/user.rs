use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_web::{delete, error, get, web, Error, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use anyhow::Result;

/// retrieve all users
#[get("/user")]
#[has_any_role("ADMIN")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    //utils::validate_api_key(&req)?;
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    let mut conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || User::find_all(&mut conn, limit, offset, q))
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

#[delete("/user")]
#[has_any_role("ADMIN")]
pub async fn delete(del_user: web::Json<utils::IdType>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    //utils::validate_api_key(&req)?;
    let mut conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || User::delete(&mut conn, del_user.id))
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
