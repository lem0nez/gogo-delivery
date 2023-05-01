pub mod db;
pub mod mutation;
pub mod query;
pub mod types;

use std::sync::Arc;

use actix_web::{dev::ServiceRequest, http::header, web::Data, HttpResponse};
use actix_web_httpauth::extractors::{
    basic::{BasicAuth, Config},
    AuthenticationError,
};
use async_graphql::{http::GraphQLPlaygroundConfig, Context, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use base64::Engine;
use log::warn;
use mutation::MutationRoot;
use query::QueryRoot;
use sha2::{Digest, Sha256};

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn request(
    schema: Data<AppSchema>,
    req: GraphQLRequest,
    auth: BasicAuth,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth)).await.into()
}

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

pub async fn playground(auth: BasicAuth) -> HttpResponse {
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

pub fn auth_from_ctx<'a>(ctx: &Context<'a>) -> &'a BasicAuth {
    ctx.data::<BasicAuth>()
        .expect("BasicAuth object isn't passed for request")
}

pub fn sha256(data: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(data);
    format!("{:x}", sha256.finalize())
}
