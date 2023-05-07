// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::env;

use log::error;
use serde::Deserialize;
use tokio_postgres::{NoTls, Row};

use crate::{
    sha256,
    types::{Category, Notification, User, ID},
};

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreviewOf {
    Category,
    Food,
}

type PostgresResult<T> = Result<T, tokio_postgres::Error>;

pub struct Client {
    client: tokio_postgres::Client,
}

impl Client {
    pub async fn connect() -> PostgresResult<Self> {
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
    ) -> PostgresResult<bool> {
        Ok(self
            .client
            .query_one(
                include_str!("sql/is_credentials_valid.sql"),
                &[&username, &sha256(password)],
            )
            .await?
            .get(0))
    }

    pub async fn user(&self, username: &str) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select_user.sql"), &[&username])
            .await
            .map(Into::into)
    }

    pub async fn notifications(&self, username: &str) -> PostgresResult<Vec<Notification>> {
        self.client
            .query(
                include_str!("sql/select_notifications.sql"),
                &[&self.user_id(username).await?],
            )
            .await
            .map(from_rows)
    }

    pub async fn categories(&self) -> PostgresResult<Vec<Category>> {
        self.client
            .query(include_str!("sql/select_categories.sql"), &[])
            .await
            .map(from_rows)
    }

    pub async fn preview(&self, of: PreviewOf, id: ID) -> PostgresResult<Vec<u8>> {
        self.client
            .query_one(
                match of {
                    PreviewOf::Category => include_str!("sql/select_category_preview.sql"),
                    PreviewOf::Food => include_str!("sql/select_food_preview.sql"),
                },
                &[&id],
            )
            .await
            .map(|row| row.get(0))
    }

    async fn user_id(&self, username: &str) -> PostgresResult<ID> {
        self.user(username).await.map(|user| user.id)
    }
}

fn from_rows<T: From<Row>>(rows: Vec<Row>) -> Vec<T> {
    rows.into_iter().map(Into::into).collect()
}
