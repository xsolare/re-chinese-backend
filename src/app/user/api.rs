use crate::app::user::model::{User};
use crate::app::user::{request::*, response::*};
use crate::middleware::auth;
use crate::AppState;
use actix_web::{web, HttpRequest, HttpResponse};

pub async fn signin(
    state: web::Data<AppState>,
    form: web::Json<Signin>,
) -> Result<HttpResponse, HttpResponse> {
    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");
    let (user, token) =
        web::block(move || User::signin(&conn, &form.user.email, &form.user.password))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().json(e.to_string())
            })?;
    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn signup(
    state: web::Data<AppState>,
    form: web::Json<Signup>,
) -> Result<HttpResponse, HttpResponse> {

    let conn = state
        .pool
        .get()
        .expect("couldn't get db connection from pool");

    let (user, token) = web::block(move || {
        User::signup(
            &conn,
            &form.user.email,
            &form.user.username,
            &form.user.password,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().json(e.to_string())
    })?;

    let res = UserResponse::from((user, token));
    Ok(HttpResponse::Ok().json(res))
}

pub async fn auth(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let user = auth::access_auth_user(&req);

    if let Some(user) = user {
        let user = UserResponse::from((user.to_owned(), user.generate_token()));
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::Ok().json({}))
    }
}