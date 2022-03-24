use crate::app;
use actix_web::web;
use actix_web::web::{delete, get, post, put};

pub fn api(cfg: &mut web::ServiceConfig){
    cfg.service(
web::scope("/api")
        .service(web::scope("/index").route("", post().to(app::index::api::index)))        
        .service(
            web::scope("/users")
                .route("/signin", post().to(app::user::api::signin))
                .route("/signup", post().to(app::user::api::signup))
                .route("/auth", get().to(app::user::api::auth)),
        ),
    );
}
