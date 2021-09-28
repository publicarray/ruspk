use crate::{api::package, models::*};
use crate::{AppData, DbConn};
use actix_web::{delete, get, post, web, Error, HttpRequest, HttpResponse};
use anyhow::Result;
extern crate serde_derive;
extern crate serde_qs as qs;
use crate::utils;
use async_std::prelude::*;
use futures::{StreamExt, TryFutureExt};
use async_tar::Archive;
use flate2::read::GzDecoder;

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

// todo optimisations
#[post("/build")]
pub async fn post(mut body: web::Payload, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    // read post data / file
    let filepath = format!("./tmp/{}", "temp.spk"); // fix me (temp name then move /upload file to cdn)
    let mut f = async_std::fs::File::create(filepath.clone()).await?;

    while let Some(chunk) = body.next().await {
        let data = chunk.unwrap();
        f.write_all(&data).await?;
    }

    // todo check file is in fact a tar archive

    // extract Info file from tar
    let mut info_contents = String::new();
    let file = async_std::fs::File::open(filepath.clone()).await?;
    let archive = Archive::new(file);
    let mut entries = archive.entries().unwrap();
    while let Some(item) = entries.next().await {
        let mut f = match item {
            Ok (i) => i,
            Err(_) => break // end of file or other error
        };

        // INFO
        debug!("extract tar: {}", f.path().unwrap().display());
        if f.path().unwrap() == async_std::path::Path::new("INFO") {
            f.read_to_string(&mut info_contents).await?;
        }

        // package.tgz
        if f.path().unwrap() == async_std::path::Path::new("package.tgz") {

            let result = f.unpack_in("./tmp/").await?;
            debug!("extracted package.tgz {}", result);

            let archive_file = std::fs::File::open("./tmp/package.tgz")?;
            let archive_buff = std::io::BufReader::new(archive_file);
            let archive_decoded = GzDecoder::new(archive_buff);
            let mut package_archive = tar::Archive::new(archive_decoded);

            let mut package_entries = package_archive.entries().unwrap();
            while let Some(package_item) = package_entries.next() {
                // let package_file = package_item.unwrap();
                let mut package_file = match package_item {
                    Ok (i) => i,
                    Err(_) => break // end of file or other error
                };
                debug!("extract package tar: {}", package_file.path().unwrap().display());
            }
        }
    }


    // convert to booleans hack
    info_contents = info_contents
        .replace("=\"yes\"", "=true")
        .replace("=\"no\"", "=false")
        .replace("=\"Yes\"", "=true")
        .replace("=\"No\"", "=false")
        .replace("=\"YES\"", "=true")
        .replace("=\"NO\"", "=false");
    // serialise info file to a struct
    let info: Info = toml::from_str(&info_contents).map_err(|_| actix_web::error::ParseError::Incomplete)?;

    // serialise info file to a struct & save info into database

    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::create_build(&conn, info))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(response))
    // Ok(HttpResponse::Ok().json(info))
}

#[delete("/build")]
// pub async fn get_builds(req: HttpRequest, json_data: web::Json<utils::Paginate>, data: web::Data<AppData>) -> Result<HttpResponse, Error>{
pub async fn delete(post_data: web::Json<utils::IdType>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::delete_build(&conn, post_data.id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(response))
}
