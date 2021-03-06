use crate::app::user::model::User;
use crate::middleware;
use crate::utils::{constants, token};
use crate::AppState;
use actix_service::{Service, Transform};
use actix_web::http::header::{self, HeaderName, HeaderValue};
use actix_web::HttpMessage;
use actix_web::{
    dev::ServiceRequest, dev::ServiceResponse, http::Method, web::Data, Error, HttpRequest,
    HttpResponse,
};
use diesel::pg::PgConnection;
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use uuid::Uuid;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Authentication;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    // S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    // S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    // type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        self: &AuthenticationMiddleware<S>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(self: &AuthenticationMiddleware<S>, mut req: ServiceRequest) -> Self::Future {
        if should_skip_verify(&req) || verify(&mut req) {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            let fut = self.service.call(req);
            Box::pin(async move {
                //TODO check issues
                let res = fut.await?;
                Ok(res)
            })
        }
    }
}

fn should_skip_verify(req: &ServiceRequest) -> bool {
    if Method::OPTIONS == *req.method() {
        return true;
    }

    for ignore_route in constants::IGNORE_AUTH_ROUTES.iter() {
        if req.path().starts_with(ignore_route) {
            return true;
        }
    }

    return false;
}

fn find_auth_user(conn: &PgConnection, user_id: Uuid) -> User {
    User::find_by_id(&conn, user_id)
}

fn verify(req: &mut ServiceRequest) -> bool {
    req.headers_mut().append(
        HeaderName::from_static("content-length"),
        HeaderValue::from_static("true"),
    );

    if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
        info!("Parsing authorization header...");
        if let Ok(authen_str) = authen_header.to_str() {
            if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                info!("Parsing token...");
                let token = authen_str[6..authen_str.len()].trim();
                match token::decode(&token) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        let user_id = claims.user_id;
                        if let Some(state) = req.app_data::<Data<AppState>>() {
                            let conn = state
                                .pool
                                .get()
                                .expect("couldn't get db connection from pool");
                            let user = find_auth_user(&conn, user_id);

                            //TODO put headers
                            // req.head()
                            // .headers()
                            // .insert(header::SET_COOKIE, HeaderValue::from_str("user"));
                            // req.head().extensions_mut().insert(user);
                        }
                        return true;
                    }
                    _ => {
                        error!("Invalid token");
                        return false;
                    }
                }
            }
        }
    };
    false
}

//TODO access_auth_user with headers
pub fn access_auth_user(req: &HttpRequest) -> Option<HeaderValue> {
    let head = req.head();
    let headers = head.headers();
    let _user = headers.get(header::SET_COOKIE);
    let auth_user = _user.map(|user| user.to_owned());

    auth_user
}
