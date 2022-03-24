use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use crate::AppState;
use crate::app::user::{request, response};

// pub async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Yep!")
// }


pub async fn index(
    // state: web::Data<AppState>,
    form: web::Json<request::Signup>,
) -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::Ok().json(form.user.username.to_string()))
}
