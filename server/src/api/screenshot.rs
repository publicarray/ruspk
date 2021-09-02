use crate::models::*;
use actix_web::{get, web, Error, HttpResponse};
use crate::{AppData, DbConn};
use anyhow::Result;
use diesel::{self, prelude::*};


fn db_get_screenshots(conn: &DbConn) -> Result<Vec<DbScreenshot>> {
    use crate::schema::screenshot;
    let p = screenshot::table
        .load::<DbScreenshot>(conn)
        .expect("Error loading screenshot from DB");
    Ok(p)
}

/// retrieve all screenshots
#[get("/screenshot")]
pub async fn get_screenshots(data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_screenshots(&conn)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}
