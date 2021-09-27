use crate::models::*;
use crate::{AppData, DbConn};
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;
extern crate serde_derive;
extern crate serde_qs as qs;
use crate::utils;

fn db_get_build(conn: &DbConn, limit: i64, offset: i64) -> Result<Vec<Build>> {
    Ok(DbBuild::find_all(conn, limit, offset)?)
}

#[get("/build")]
// pub async fn get_builds(req: HttpRequest, json_data: web::Json<utils::Paginate>, data: web::Data<AppData>) -> Result<HttpResponse, Error>{
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset) = utils::paginate_qs(req.query_string());
    // let (q_limit, q_offset) = utils::paginate_qs(req.query_string());
    // let limit = json_data.size.unwrap_or(q_limit);
    // let offset = json_data.page.unwrap_or(q_offset);

    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_build(&conn, limit, offset))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
