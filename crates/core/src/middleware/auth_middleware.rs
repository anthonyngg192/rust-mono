use std::rc::Rc;

use actix_web::body::EitherBody;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse};
use futures::future::{ok, LocalBoxFuture, Ready};

use crate::database::Database;
use crate::utils::token_util;
pub struct Authentication;

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let mut headers = req.headers().clone();

        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        Box::pin(async move {
            let mut authenticate_pass: bool = false;
            if let Some(db) = req.app_data::<Data<Database>>() {
                if let Some(authen_header) = req.headers().get("Authorization") {
                    if let Ok(authen_str) = authen_header.to_str() {
                        if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                            let token = authen_str[6..authen_str.len()].trim();
                            if let Ok(token_data) = token_util::decode_token(token.to_string()) {
                                if token_util::verify_token(&token_data, db.clone())
                                    .await
                                    .is_ok()
                                {
                                    authenticate_pass = true;
                                }
                            }
                        }
                    }
                }
            }

            if !authenticate_pass {
                let (request, _pl) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .json("Invalid token, please login again")
                    .map_into_right_body();

                return Ok(ServiceResponse::new(request, response));
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }

    forward_ready!(service);
}

impl<S, B> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}
