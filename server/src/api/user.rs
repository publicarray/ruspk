use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_web::delete;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;
use actix_web_grants::proc_macro::has_any_role;

/// retrieve all users
#[get("/user")]
#[has_any_role("ADMIN")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    //utils::validate_api_key(&req)?;
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || User::find_all(&conn, limit, offset, q))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/user")]
#[has_any_role("ADMIN")]
pub async fn delete(del_user: web::Json<utils::IdType>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    //utils::validate_api_key(&req)?;
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || User::delete(&conn, del_user.id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
