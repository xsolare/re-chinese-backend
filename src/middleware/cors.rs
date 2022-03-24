use crate::utils::constants::env_key;
use actix_cors::Cors;
use actix_web::http;
use std::env;

pub fn cors() -> Cors {
    let frontend_origin = env::var(env_key::FRONTEND_ORIGIN).ok();
    let frontend_origin = frontend_origin.ok_or("*").unwrap();

    let cors = Cors::default()
        .allowed_origin(&frontend_origin)
        .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    cors
}
