use crate::models::*;
use actix_web::{get, post, web, Error, HttpResponse};
use crate::{AppData, DbConn};
use anyhow::Result;

fn db_get_packages(conn: &DbConn) -> Result<Vec<Package>> {
    Ok(DbPackage::find_all(&conn)?)
}

/// retrieve all packages
#[get("/package")]
pub async fn get_packages(data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_packages(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}

/// create a new package
#[post("/package")]
pub async fn new_package(data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_packages(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}
