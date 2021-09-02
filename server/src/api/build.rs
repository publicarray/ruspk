use crate::models::*;
use actix_web::{get, web, Error, HttpResponse};
use crate::{AppData, DbConn};
use anyhow::Result;

fn db_get_build(conn: &DbConn) -> Result<Vec<Build>> {
    Ok(DbBuild::find_all(conn)?)
}

/// retrieve all build
#[get("/build")]
pub async fn get_builds(data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_build(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}
