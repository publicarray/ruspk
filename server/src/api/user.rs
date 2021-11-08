use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_identity::Identity;
use actix_web::delete;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;

/// retrieve all users
#[get("/user")]
pub async fn get_all(req: HttpRequest, id: Identity, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    debug!("{:?}", id.identity());
    if let Some(_id) = id.identity() {
        let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
        let conn = data.pool.get().expect("couldn't get db connection from pool");
        let response = web::block(move || User::find_all(&conn, limit, offset, q))
            .await
            .map_err(|e| {
                debug!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[delete("/user")]
pub async fn delete(
    del_user: web::Json<utils::IdType>,
    id: Identity,
    data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    debug!("{:?}", id.identity());
    if let Some(_id) = id.identity() {
        let conn = data.pool.get().expect("couldn't get db connection from pool");
        let response = web::block(move || User::delete(&conn, del_user.id))
            .await
            .map_err(|e| {
                debug!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(response))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}
