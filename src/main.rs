// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use env_logger::Env;

use gogo_delivery::{db, mutation::MutationRoot, query::QueryRoot, rest};

const SERVER_ADDRESS: (&str, u16) = ("0.0.0.0", 5000);
const CORS_MAX_AGE_SECS: usize = 3600;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("INFO"));

    let db = Arc::new(db::Client::connect().await?);
    let schema = Schema::build(
        QueryRoot::new(Arc::clone(&db)),
        MutationRoot::new(Arc::clone(&db)),
        EmptySubscription,
    )
    .finish();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["POST"])
            .allowed_headers(vec![
                header::ACCEPT,
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
            ])
            .max_age(CORS_MAX_AGE_SECS);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(Arc::clone(&db)))
            .configure(rest::configure_service)
    });
    server.bind(SERVER_ADDRESS)?.run().await.map_err(Into::into)
}
