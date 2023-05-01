// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::env;

use log::error;
use tokio_postgres::{Error, NoTls};

use crate::{sha256, types::User};

pub struct Client {
    client: tokio_postgres::Client,
}

impl Client {
    pub async fn connect() -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(
            &env::var("DB_CONNECTION_STRING")
                .expect("environment variable DB_CONNECTION_STRING isn't defined"),
            NoTls,
        )
        .await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                error!("Unable to establish connection to database: {e}");
            }
        });
        Ok(Self { client })
    }

    pub async fn is_credentials_valid(
        &self,
        username: &str,
        password: &str,
    ) -> Result<bool, Error> {
        Ok(self
            .client
            .query_one(
                include_str!("sql/is_credentials_valid.sql"),
                &[&username, &sha256(password)],
            )
            .await?
            .get(0))
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>, Error> {
        Ok(self
            .client
            .query_opt(include_str!("sql/select_user.sql"), &[&username])
            .await?
            .map(Into::into))
    }
}
