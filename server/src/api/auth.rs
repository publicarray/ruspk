use crate::AppData;
use crate::{claims, models::*};
use actix_web::{error, get, post, web, Error, HttpResponse};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    username: Option<String>,
    email: Option<String>,
    // token: Option<String>,
    password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewResetRequest {
    email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResetRequest {
    token: String,
    password: String,
    password_confirmation: String,
}

use crate::claims::Claims;
use actix_web::dev::ServiceRequest;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

// https://github.com/DDtKey/actix-web-grants/blob/main/examples/jwt-httpauth/src/main.rs
pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    let result = claims::decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            trace!("{:?}", claims);
            req.attach(claims.permissions);
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => Err((e, req)),
    }
}

// Login handler for generating a token.
// cURL example:
// ```sh
//  curl --location --request POST 'http://localhost:8080/token' \
//   --header 'Content-Type: application/json' \
//   --data-raw '{
//       "username": "JohnDoe",
//       "permissions": ["ROLE_ADMIN"]
//   }'
// ```
#[post("/login")]
pub async fn login(info: web::Json<Auth>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let user_info = info.into_inner();
    debug!("{:?}", user_info);
    let mut conn = data.pool.get().expect("couldn't get db connection from pool");
    let (user, roles) =
        web::block(move || User::login(&mut conn, &user_info.username, &user_info.email, &user_info.password))
            .await
            .map_err(|e| {
                debug!("{}", e);
                error::ErrorInternalServerError(e)
            })?
            .map_err(|e| {
                debug!("{}", e);
                error::ErrorInternalServerError(e)
            })?;

    // [(role_ + role.name).to_uppercase()]
    let role_str = roles
        .into_iter()
        .map(|x| ("role_".to_owned() + x.name.as_str()).to_uppercase())
        .collect::<Vec<_>>();
    debug!("{:?}, {:?}", &user, &role_str);
    // Create a JWT
    let claims = Claims::new(user.username, role_str);
    let jwt = claims::create_jwt(claims)?;

    // Return token for work with example handlers
    Ok(HttpResponse::Ok().json(jwt))
}

#[post("/newreset")]
pub async fn new_reset(info: web::Json<NewResetRequest>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let user_info = info.into_inner();
    debug!("{:?}", user_info);
    let mut conn = data.pool.get().expect("couldn't get db connection from pool");
    let (_user, _roles) = web::block(move || User::send_reset_link(&mut conn, &user_info.email))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;

    // Return token for work with example handlers
    Ok(HttpResponse::Ok().json(""))
}

#[get("/profile")]
pub async fn profile(data: web::Data<AppData>, credentials: BearerAuth) -> Result<HttpResponse, Error> {
    let claims = claims::decode_jwt(credentials.token())?;
    info!("{:?}", &claims);
    let mut conn = data.pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || User::get(&mut conn, claims.username))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;
    info!("{:?}", user);
    Ok(HttpResponse::Ok().json(user))
}
