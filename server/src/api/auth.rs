use crate::AppData;
use crate::{claims, models::*};
use actix_web::{error, post, web, Error, HttpResponse};
use anyhow::Result;
// use actix_web_grants::proc_macro::has_any_role;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    username: Option<String>,
    email: Option<String>,
    // token: Option<String>,
    password: String,
}

use crate::claims::Claims;
use actix_web::dev::ServiceRequest;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    // We just get permissions from JWT
    let claims = claims::decode_jwt(credentials.token())?;
    debug!("{:?}", &claims);
    req.attach(claims.permissions);
    Ok(req)
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

// #[post("/profile")]
// pub async fn profile(auth: web::Json<Auth>, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
//     let conn = data.pool.get().expect("couldn't get db connection from pool");
//     debug!("{:?}", auth.password);
//     debug!("{:?}", auth.username);
//     debug!("{:?}", auth.email);
//     let user = web::block(move || User::login(&conn, &auth.username, &auth.email, &auth.password))
//         .await
//         .map_err(|e| {
//             debug!("{}", e);
//             HttpResponse::Unauthorized().finish()
//         })?;

//     debug!("{:?}", user);
//     Ok(HttpResponse::Ok().json(user))
// }
