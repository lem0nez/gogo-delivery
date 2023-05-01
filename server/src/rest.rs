// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use actix_web::{
    get,
    http::header,
    post,
    web::{Data, ServiceConfig},
    HttpResponse,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use base64::Engine;

use crate::{auth_validator, AppSchema};

pub fn configure_service(config: &mut ServiceConfig) {
    config.service(request).service(playground);
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
