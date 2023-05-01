// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

pub mod db;
pub mod mutation;
pub mod query;
pub mod rest;
pub mod types;

use std::sync::Arc;

use actix_web::{dev::ServiceRequest, web::Data};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config},
    AuthenticationError,
};
use async_graphql::{Context, EmptySubscription, Schema};
use log::warn;
use mutation::MutationRoot;
use query::QueryRoot;
use sha2::{Digest, Sha256};

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn auth_validator(
    req: ServiceRequest,
    auth: BasicAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let user = auth.user_id();
    if let Some(db) = req.app_data::<Data<Arc<db::Client>>>() {
        let result = db
            .is_credentials_valid(user, auth.password().unwrap_or_default())
            .await;
        if result.unwrap_or(false) {
            return Ok(req);
        }
    }

    warn!("User {user} failed to authenticate");
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    Err((AuthenticationError::from(config).into(), req))
}

pub fn auth_from_ctx<'a>(ctx: &Context<'a>) -> &'a BasicAuth {
    ctx.data::<BasicAuth>()
        .expect("BasicAuth object isn't passed for request")
}

pub fn sha256(data: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(data);
    format!("{:x}", sha256.finalize())
}
