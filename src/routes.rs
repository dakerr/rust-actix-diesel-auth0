//! Place all Actix routes here, multiple route configs can be used and
//! combined.

use crate::auth::validator;
use crate::handlers::{
    health::get_health,
    user::{get_user, get_users},
};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    cfg
        // Healthcheck
        .route("/health", web::get().to(get_health))
        // /api/v1/ routes
        .service(
            web::scope("/api/v1")
            // USER routes
            .wrap(auth)
            .service(
                web::scope("/user")
                    .route("/{id}", web::get().to(get_user))
                    .route("", web::get().to(get_users))
            ),
        );
}