use crate::models::*;
use actix_web::{get, web, Error, HttpResponse};
use crate::{AppData, DbConn};
use anyhow::Result;
use diesel::{self, prelude::*};


fn db_get_versions(conn: &DbConn) -> Result<Vec<DbVersion>> {
    use crate::schema::version;
    let p = version::table
        .load::<DbVersion>(conn)
        .expect("Error loading versions from DB");
    Ok(p)
}

/// retrieve all versions
#[get("/version")]
pub async fn get_versions(data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_versions(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}
