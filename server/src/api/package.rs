use crate::models::*;
use crate::utils;
use crate::{AppData, DbConn};
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;

fn db_get_packages(conn: &DbConn, limit: i64, offset: i64) -> Result<Vec<Package>> {
    Ok(DbPackage::find_all(conn, limit, offset)?)
}

/// retrieve all packages
#[get("/package")]
pub async fn get_packages(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset) = utils::paginate_qs(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_packages(&conn, limit, offset))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}

/// create a new package
#[post("/package")]
pub async fn create_package(_: web::Data<AppData>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(""))
}
