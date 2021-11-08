use crate::models::*;
use crate::AppData;
use actix_identity::Identity;
use actix_web::{post, web, Error, HttpResponse};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    username: Option<String>,
    email: Option<String>,
    // token: Option<String>,
    password: String,
}

#[post("/login")]
pub async fn login(auth: web::Json<Auth>, id: Identity, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    debug!("{:?}", auth.password);
    debug!("{:?}", auth.username);
    debug!("{:?}", auth.email);
    let response = web::block(move || User::login(&conn, &auth.username, &auth.email, &auth.password))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;
    id.remember(response.clone());
    debug!("{:?}", id.identity());
    Ok(HttpResponse::Ok().json(response))
}

#[post("/logout")]
pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}
