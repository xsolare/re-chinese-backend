//* Example
use crate::app::user::{model, request, response};

use crate::AppState;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

use diesel::prelude::*;
use diesel::sql_query;

// pub async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

// pub async fn index(form: web::Json<request::Signup>) -> impl Responder {
//     HttpResponse::Ok().json(form.user.username.to_string())
// }

pub async fn index(state: web::Data<AppState>) -> impl Responder {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let users = sql_query("SELECT * FROM users")
        .load::<model::User>(&conn)
        .unwrap();

    HttpResponse::Ok().json(users)
}
