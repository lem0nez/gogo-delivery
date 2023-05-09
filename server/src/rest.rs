// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use actix_web::{
    get,
    http::header,
    post,
    web::{Data, Query, ServiceConfig},
    HttpResponse,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use base64::Engine;
use log::info;
use serde::Deserialize;

use crate::{
    auth_validator,
    db::{self, PreviewOf},
    types::{User, ID},
    AppSchema,
};

pub fn configure_service(config: &mut ServiceConfig) {
    config
        .service(request)
        .service(playground)
        .service(preview)
        .service(sign_up);
}

#[post("/", wrap = "HttpAuthentication::basic(auth_validator)")]
async fn request(schema: Data<AppSchema>, req: GraphQLRequest, auth: BasicAuth) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth)).await.into()
}

#[get("/", wrap = "HttpAuthentication::basic(auth_validator)")]
async fn playground(auth: BasicAuth) -> HttpResponse {
    let credentials = format!("{}:{}", auth.user_id(), auth.password().unwrap_or_default());
    let auth_header = "Basic ".to_string()
        + &base64::engine::general_purpose::STANDARD_NO_PAD.encode(credentials);

    let config = GraphQLPlaygroundConfig::new("/")
        .subscription_endpoint("/")
        .with_header(header::AUTHORIZATION.as_str(), &auth_header);
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(async_graphql::http::playground_source(config))
}

#[derive(Deserialize)]
struct PreviewQuery {
    of: PreviewOf,
    id: ID,
}

#[get("/preview", wrap = "HttpAuthentication::basic(auth_validator)")]
async fn preview(query: Query<PreviewQuery>, db: Data<Arc<db::Client>>) -> HttpResponse {
    db.preview(query.of, query.id)
        .await
        .map(|bytes| HttpResponse::Ok().content_type("image/jpeg").body(bytes))
        .unwrap_or_else(|err| HttpResponse::BadRequest().body(err.to_string()))
}

#[post("/sign_up")]
async fn sign_up(
    mut user: Query<User>,
    auth: BasicAuth,
    db: Data<Arc<db::Client>>,
) -> HttpResponse {
    let username = auth.user_id();
    user.username = username.to_string();
    if let Some(password) = auth.password() {
        user.password = password.to_string();
    }
    db.add_user(user.into_inner())
        .await
        .map(|id| {
            info!("New customer {username} signed up");
            HttpResponse::Ok().body(id.to_string())
        })
        .unwrap_or_else(|err| HttpResponse::BadRequest().body(err.to_string()))
}
