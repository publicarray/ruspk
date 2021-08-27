use actix_web::{post, web::Path, HttpResponse};

/// add build to a version `/packages`
#[post("/packages")]
pub async fn new_build(path: Path<(String,)>) -> HttpResponse {
    HttpResponse::Created()
        .content_type("application/json")
        .json("Ok")
}
