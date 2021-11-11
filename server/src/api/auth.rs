use crate::{claims, models::*};
use crate::AppData;
use actix_web::{post, web, Error, HttpResponse};
use anyhow::Result;
// use actix_web_grants::proc_macro::{has_any_role, has_permissions};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    username: Option<String>,
    email: Option<String>,
    // token: Option<String>,
    password: String,
}

use crate::claims::Claims;
// mod claims;
use actix_web::dev::ServiceRequest;
// use actix_web::{post, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_grants::permissions::AttachPermissions;

#[derive(Deserialize)]
pub struct UserPermissions {
    pub username: String,
    pub permissions: Vec<String>,
}

// An additional handler for generating a token.
// Thus, you can try to create your own tokens and check the operation of the permissions system.
// cURL example:
// ```sh
//  curl --location --request POST 'http://localhost:8080/token' \
//   --header 'Content-Type: application/json' \
//   --data-raw '{
//       "username": "Lorem-Ipsum",
//       "permissions": ["OP_GET_SECURED_INFO"]
//   }'
// ```
#[post("/token")]
pub async fn create_token(info: web::Json<Auth>,  data: web::Data<AppData>) -> Result<String, Error> {
    let user_info = info.into_inner();
    debug!("{:?}", user_info.password);
    debug!("{:?}", user_info.username);
    debug!("{:?}", user_info.email);
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let (user, roles) = web::block(move || User::login(&conn, &user_info.username, &user_info.email, &user_info.password))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;

    debug!("{:?}", user);

    // Create a JWT
    // let claims = Claims::new(user.username, user.permissions);
    // let claims = Claims::new(user.username, vec!["ROLE_ADMIN".to_string()]);
    let claims = Claims::new(user.username, roles);
    let jwt = claims::create_jwt(claims)?;

    // Return token for work with example handlers
    Ok(jwt)
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    // We just get permissions from JWT
    let claims = claims::decode_jwt(credentials.token())?;
    debug!("{:?}", &claims);
    dbg!(&claims);
    req.attach(claims.permissions);
    Ok(req)
}

#[post("/login")]
pub async fn login(auth: web::Json<Auth>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    debug!("{:?}", auth.password);
    debug!("{:?}", auth.username);
    debug!("{:?}", auth.email);
    let user = web::block(move || User::login(&conn, &auth.username, &auth.email, &auth.password))
        .await
        .map_err(|e| {
            debug!("{}", e);
            HttpResponse::Unauthorized().finish()
        })?;

    debug!("{:?}", user);
    Ok(HttpResponse::Ok().json(user))
}
