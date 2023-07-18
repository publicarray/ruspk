use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::DbId;
#[derive(Deserialize)]
pub struct IdType {
    pub id: DbId,
}

pub fn read_file(file_name: &str) -> Result<String> {
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn str_to_u64(str: &str) -> u64 {
    trace!("str_to_u64: {}", str);
    str.parse()
        .expect("Input is not a valid positive digit that fits into u64")
}

#[derive(Deserialize)]
pub struct Paginate {
    pub page: Option<i64>,
    pub size: Option<i64>,
}

pub fn fuzzy_search(q: &str) -> String {
    let replaced = q.replace('-', " ").replace(' ', "%");
    format!("%{}%", replaced)
}
/// retrieve HTTP GET Parameters for pagination
extern crate serde_derive;
extern crate serde_qs as qs;
pub fn handle_query_parameters(query_str: &str) -> (i64, i64, String) {
    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Parameters {
        page: Option<i64>,
        size: Option<i64>,
        q: Option<String>,
    }

    // if strings are not found use defaults
    let params: Parameters = qs::from_str(query_str).unwrap_or(Parameters {
        page: Some(0),
        size: Some(20),
        q: Some(" ".to_string()),
    });
    let mut offset = params.page.unwrap_or(1); //defaults if not provided
    let mut limit = params.size.unwrap_or(20); //defaults if not provided
    let query = params.q.unwrap_or_else(|| " ".to_string()); //defaults if not provided

    // check fot negatives
    if offset <= 0 {
        offset = 1
    }
    if limit <= 0 {
        limit = 1
    // check that query is reasonable and does not overload the database
    } else if limit > 100 {
        limit = 100
    }

    (limit, (offset - 1) * limit, query)
}


use crate::models;
use crate::AppData;
use actix_web::{error, error::Error, http::header::Header, web, HttpRequest};
use actix_web_httpauth::headers::authorization::Authorization;
use actix_web_httpauth::headers::authorization::Basic;
use models::User;
pub fn validate_api_key(req: &HttpRequest) -> Result<(), Error> {
    let auth = Authorization::<Basic>::parse(req)?;
    let credentials = auth.into_scheme();
    let api_key = credentials.user_id();

    let app_data = req.app_data::<web::Data<AppData>>().unwrap();
    let mut conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let user = User::validate_api_key(&mut conn, api_key.to_string())
        .map_err(|_| error::ErrorUnauthorized("\
        {
            \"message\": \"The server could not verify that you are authorized to access the URL requested. You either supplied the wrong credentials (e.g. a bad password), or your browser doesn't understand how to supply the credentials required.\"
        }
    "))?;

    debug!("{:?}", user);
    Ok(())
}

use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};
/// Create simple rustls client config from root certificates.
pub fn rustls_client_config() -> ClientConfig {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| OwnedTrustAnchor::from_subject_spki_name_constraints(ta.subject, ta.spki, ta.name_constraints)),
    );

    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use crate::SMTP_SERVER;
use crate::SMTP_PORT;
use crate::SMTP_USERNAME;
use crate::SMTP_PASSWORD;
use crate::SMTP_FROM;

pub fn send_email(message: String, subject: &str, to: &str) {

    let email = Message::builder()
        .from((*SMTP_FROM).parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject.to_string())
        .header(ContentType::TEXT_PLAIN)
        .body(message)
        .unwrap();

    let creds = Credentials::new((*SMTP_USERNAME).clone(), (*SMTP_PASSWORD).clone());

    // Open a remote connection to SMTP server
    //let mailer = SmtpTransport::starttls_relay("")
    let mailer = SmtpTransport::relay(&SMTP_SERVER)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => debug!("Email sent successfully! {}", subject),
        Err(e) => warn!("Could not send email: {:?}", e),
    }
}
