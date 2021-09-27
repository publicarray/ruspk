use crate::models::*;
use crate::utils;
use crate::{AppData, DbConn};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;

fn db_get_screenshots(conn: &DbConn, limit: i64, offset: i64) -> Result<Vec<Screenshot>> {
    Ok(DbScreenshot::find_all(conn, limit, offset)?)
}

/// retrieve all screenshots
#[get("/screenshot")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset) = utils::paginate_qs(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_screenshots(&conn, limit, offset))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
