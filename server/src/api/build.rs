use std::io::{Read, Write};

use crate::AppData;
use crate::{models::*, DbId};
use actix_web::{delete, error, get, post, put, web, Error, HttpRequest, HttpResponse};
use anyhow::{Context, Result};
use openpgp::serialize::stream::{Armorer, Message, Signer};
use openpgp::{parse::Parse};
extern crate serde_derive;
extern crate serde_qs as qs;
use crate::utils;
use crate::PGP_KEY_PATH;
use crate::STORAGE_PATH;
use crate::STORAGE_TYPE;
use actix_web_grants::proc_macro::has_any_role;
use async_std::path::Path;
use async_std::{io, prelude::*};
use async_tar::Archive;
use futures::StreamExt;
use regex::Regex;

#[get("/api/build")]
// pub async fn get_builds(req: HttpRequest, json_data: web::Json<utils::Paginate>, data: web::Data<AppData>) -> Result<HttpResponse, Error>{
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    // let (q_limit, q_offset) = utils::paginate_qs(req.query_string());
    // let limit = json_data.size.unwrap_or(q_limit);
    // let offset = json_data.page.unwrap_or(q_offset);

    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::find_all(&conn, limit, offset, q))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(response))
}

/// Signs the given message.
// https://gitlab.com/sequoia-pgp/sequoia/-/tree/main/openpgp/examples
fn sign(
    p: &dyn openpgp::policy::Policy,
    sink: &mut (dyn Write + Send + Sync),
    plaintext: &[u8],
    tsk: &openpgp::Cert,
) -> openpgp::Result<()> {
    // Get the keypair to do the signing from the Cert.
    let keypair = tsk
        .keys()
        .unencrypted_secret()
        .with_policy(p, None)
        .supported()
        .alive()
        .revoked(false)
        .for_signing()
        .next()
        .unwrap()
        .key()
        .clone()
        .into_keypair()?;

    // Start streaming an OpenPGP message.
    let message = Message::new(sink);

    // use ASCII Formatting
    let message = Armorer::new(message).kind(openpgp::armor::Kind::Signature).build()?;

    // We want to sign a literal data packet using detached signature(s).
    let mut signer = Signer::new(message, keypair)
        .detached()
        .build()
        .context("Failed to create signer")?;

    // Sign the data.
    signer.write_all(plaintext).context("Failed to sign data")?;

    // Finalize the OpenPGP message to make sure that all data is written.
    signer.finalize().context("Failed to write data")?;

    Ok(())
}

// todo optimisations
#[post("/api/build")]
pub async fn post(
    req: HttpRequest,
    mut body: web::Payload,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, Error> {
    utils::validate_api_key(&req)?;
    // read post data / file
    let tmp_dir = tempfile::TempDir::new()?;
    let filepath = tmp_dir.path().join("temp.spk"); // fix me (temp name then move /upload file to cdn)
    let mut f = async_std::fs::File::create(filepath.clone()).await?;

    //-- use memory vs store directly to disk
    // let mut buf = web::BytesMut::new();
    // while let Some(chunk) = body.next().await {
    //     let chunk = chunk?;
    //     buf.extend_from_slice(&chunk);
    // }
    // f.write_all(&buf).await?;
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        f.write_all(&chunk).await?;
    }
    f.sync_all().await?;
    debug!("finished reading body");

    // todo check file is in fact a tar archive

    let mut to_sign = Vec::new();
    // extract Info file from tar
    let mut info_contents = String::new();
    let file = async_std::fs::File::open(filepath.clone()).await?;
    let archive = Archive::new(file);

    let mut install_wizard = false;
    let mut uninstall_wizard = false;
    let mut upgrade_wizard = false;
    let mut entries = archive.entries().unwrap();
    while let Some(item) = entries.next().await {
        let mut f = item.unwrap();
        debug!("extract tar: {}", f.path().unwrap().display());
        if f.path().unwrap() == Path::new("INFO") {
            f.read_to_string(&mut info_contents).await?;
            f.read_to_end(&mut to_sign).await?;
        }
        if f.path().unwrap() == Path::new("LICENSE") {
            f.read_to_end(&mut to_sign).await?;
        }
        if f.path().unwrap() == Path::new("PACKAGE_ICON_256.PNG") {
            f.unpack_in(tmp_dir.path()).await?;
        }
        if Regex::new(r"^PACKAGE_ICON(?:_(?P<size>120|256))?\.PNG$")
            .unwrap()
            .is_match(f.path().unwrap().to_str().unwrap())
        {
            f.read_to_end(&mut to_sign).await?;
        }
        if f.path().unwrap() == Path::new("WIZARD_UIFILES/install_uifile") {
            install_wizard = true;
        }
        if f.path().unwrap() == Path::new("WIZARD_UIFILES/uninstall_uifile") {
            uninstall_wizard = true;
        }
        if f.path().unwrap() == Path::new("WIZARD_UIFILES/upgrade_uifile") {
            upgrade_wizard = true;
        }
        if Regex::new(r"^WIZARD_UIFILES/(?P<process>install|upgrade|uninstall)_uifile(?:_[a-z]{3})?(?:\.sh)?$")
            .unwrap()
            .is_match(f.path().unwrap().to_str().unwrap())
        {
            f.read_to_end(&mut to_sign).await?;
        }
        if Regex::new(r"^conf/.+$")
            .unwrap()
            .is_match(f.path().unwrap().to_str().unwrap())
        {
            f.read_to_end(&mut to_sign).await?;
        }
        if f.path().unwrap() == Path::new("package.tgz") {
            f.read_to_end(&mut to_sign).await?;
        }
        if Regex::new(r"^scripts/.+$")
            .unwrap()
            .is_match(f.path().unwrap().to_str().unwrap())
        {
            f.read_to_end(&mut to_sign).await?;
        }
    }

    let tsk = openpgp::Cert::from_file(&*PGP_KEY_PATH)
        .context("Failed to read key")
        .unwrap();
    let p = &crate::openpgp::policy::StandardPolicy::new();
    // syno_signature.asc
    // let sig_buf = String::new();
    //let sig_buf = Vec::new();
    //let mut signature_file = std::io::Cursor::new(sig_buf);
    let signature_filepath = tmp_dir.path().join("syno_signature.asc");
    let mut signature_file = std::fs::File::create("syno_signature.asc")?;
    sign(p, &mut signature_file, &to_sign, &tsk);

    //let signature = String::from_utf8(sig_buf).unwrap();
    let mut signature = String::new();
    let mut sig_file = std::fs::File::open("syno_signature.asc")?;
    //signature_file.read_to_string(&mut signature)?;
    sig_file.read_to_string(&mut signature)?;

    let client = awc::Client::builder()
        .connector(awc::Connector::new().rustls(std::sync::Arc::new(utils::rustls_config())))
        .finish();

    debug!("signature:{}", signature);
    let res = client
        .post(&*crate::GNUPG_TIMESTAMP_URL)
        .insert_header(("User-Agent", "ruspk/1.0"))
        .insert_header(("Content-Type", "multipart/form-data; boundary=X-BOUNDARY"))
        .send_body(format!(
            "{}{}{}",
            "--X-BOUNDARY\r\nContent-Disposition: form-data; name=\"file\"; filename=\"syno_signature.asc\"\r\n",
            signature,
            "\r\n--X-BOUNDARY--\r\n"
        ))
        .await;

    debug!("Response: {:?}", res);
    if res.is_ok() {
        let body = res.unwrap().body().await?;
        if body.is_ascii() {
            let body_str = std::str::from_utf8(&body).unwrap();
            debug!("Response: {}", body_str);
            let mut signature_file = std::fs::File::create("syno_signature2.asc")?;
            signature_file.write_all(&body)?;

            let file1 = std::fs::File::open(filepath.clone())?; // new write file handler
            let mut input = tar::Archive::new(file1);

            let filepath_tmp = tmp_dir.path().join("temp2.spk");
            let file = std::fs::File::create(filepath_tmp.clone())?; // new write file handler
            let mut builder = tar::Builder::new(file);
            builder.append_archive(&mut input).unwrap();
            builder
                .append_file(
                    "syno_signature.asc",
                    &mut std::fs::File::open("syno_signature2.asc").unwrap(),
                )
                .unwrap();
            builder.finish().unwrap();
            debug!("copy archive");
            async_std::fs::copy(filepath_tmp, filepath.clone()).await?;
        }
    }

    // convert to booleans hack
    info_contents = info_contents
        .replace("=\"yes\"", "=true")
        .replace("=\"Yes\"", "=true")
        .replace("=\"YES\"", "=true")
        .replace("=\"no\"", "=false")
        .replace("=\"No\"", "=false")
        .replace("=\"NO\"", "=false");

    // serialise info file to a struct
    let info: Info = toml::from_str(&info_contents).map_err(|_| actix_web::error::ParseError::Incomplete)?;
    let icon256path = tmp_dir.path().join("PACKAGE_ICON_256.PNG");
    // move file
    if *STORAGE_TYPE == "filesystem" && !STORAGE_PATH.is_empty() {
        // path / package name / package revision
        let file_path_str = format!(
            "{}/{}/{}",
            &*STORAGE_PATH,
            info.package,
            info.version.split('-').collect::<Vec<&str>>()[1]
        );
        let file_path = Path::new(&file_path_str);
        if let Err(e) = async_std::fs::create_dir_all(file_path).await {
            if e.kind() != io::ErrorKind::AlreadyExists {
                panic!("{:?}", e)
            }
        }

        let new_filepath = file_path.join(format!(
            "{}.v{}.f{}[{}].spk",
            info.package,
            info.version.split('-').collect::<Vec<&str>>()[1], // package revision
            info.os_min_ver.split('-').collect::<Vec<&str>>()[1], // firmware build
            info.arch.replace(' ', "-")
        ));

        debug!("rename: {:?}->{:?}", filepath, new_filepath);
        //async_std::fs::rename(filepath, new_filepath).await?; // /tmp is in memory (tmpfs) and therefore a different filesystem
        async_std::fs::copy(filepath, new_filepath).await?;
        async_std::fs::copy(icon256path, file_path.join("icon256.png")).await?;
    }

    // serialise info file to a struct & save info into database
    //    let response = "not saved, please uncomment me";
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response =
        web::block(move || DbBuild::create_build(&conn, info, install_wizard, uninstall_wizard, upgrade_wizard))
            .await
            .map_err(|e| {
                debug!("{}", e);
                error::ErrorInternalServerError(e)
            })?
            .map_err(|e| {
                debug!("{}", e);
                // error::ErrorConflict(e)
                error::ErrorInternalServerError(e)
            })?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/build")]
// pub async fn get_builds(req: HttpRequest, json_data: web::Json<utils::Paginate>, data: web::Data<AppData>) -> Result<HttpResponse, Error>{
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete(post_data: web::Json<utils::IdType>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::delete(&conn, post_data.id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/build/{id}")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete_id(path: web::Path<i32>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::delete(&conn, id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Deserialize)]
pub struct BuildActive {
    id: DbId,
    active: bool,
}

// #[put("/build")]
#[put("/build/active")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN", "DEVELOPER")]
pub async fn active(post_data: web::Json<BuildActive>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbBuild::active(&conn, post_data.id, post_data.active))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;

    Ok(HttpResponse::Ok().json(response))
}
