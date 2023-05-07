// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::{collections::HashMap, env};

use anyhow::anyhow;
use log::error;
use serde::Deserialize;
use tokio_postgres::{NoTls, Row};

use crate::{
    sha256,
    types::{
        Address, Category, Favorite, Food, IndexedFavorite, IndexedFood, Notification, SortFoodBy,
        SortOrder, User, ID,
    },
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

    pub async fn user_by_name(&self, username: &str) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select_user.sql"), &[&username])
            .await
            .map(Into::into)
    }

    pub async fn user_notifications(&self, username: &str) -> PostgresResult<Vec<Notification>> {
        self.client
            .query(
                include_str!("sql/select_user_notifications.sql"),
                &[&self.user_id_by_name(username).await?],
            )
            .await
            .map(from_rows)
    }

    pub async fn user_addresses(&self, username: &str) -> PostgresResult<Vec<Address>> {
        self.client
            .query(
                include_str!("sql/select_user_addresses.sql"),
                &[&self.user_id_by_name(username).await?],
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

    pub async fn food_in_category(
        &self,
        category_id: ID,
        sort_by: SortFoodBy,
        sort_order: SortOrder,
    ) -> PostgresResult<Vec<IndexedFood>> {
        let mut food = self
            .client
            .query(
                include_str!("sql/select_food_in_category.sql"),
                &[&category_id],
            )
            .await
            .map(from_rows)?;
        food.sort_by(|lhs, rhs| sort_by.cmp(lhs, rhs));
        if let SortOrder::Descending = sort_order {
            food.reverse();
        }
        Ok(food)
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

    pub async fn user_favorites(&self, username: &str) -> anyhow::Result<Vec<Favorite>> {
        let mut food: HashMap<_, _> = self
            .food()
            .await?
            .into_iter()
            .map(|food| (food.indexed_food.id, food))
            .collect();
        let indexed_favorites: Vec<IndexedFavorite> = self
            .client
            .query(
                include_str!("sql/select_user_favorites.sql"),
                &[&self.user_id_by_name(username).await?],
            )
            .await
            .map(from_rows)?;
        let mut favorites = Vec::with_capacity(indexed_favorites.capacity());

        for indexed_favorite in indexed_favorites {
            favorites.push(Favorite {
                food: food
                    .remove(&indexed_favorite.food_id)
                    .ok_or(anyhow!("database was changed during data merging"))?,
                indexed_favorite,
            })
        }
        Ok(favorites)
    }

    async fn user_id_by_name(&self, username: &str) -> PostgresResult<ID> {
        self.user_by_name(username).await.map(|user| user.id)
    }

    async fn food(&self) -> anyhow::Result<Vec<Food>> {
        let categories: HashMap<_, _> = self
            .categories()
            .await?
            .into_iter()
            .map(|category| (category.id, category))
            .collect();
        let indexed_food: Vec<IndexedFood> = self
            .client
            .query(include_str!("sql/select_food.sql"), &[])
            .await
            .map(from_rows)?;

        let mut food = Vec::with_capacity(indexed_food.capacity());
        // Using loop instead of closure because we must be able to propage an error.
        for indexed_food in indexed_food {
            food.push(Food {
                category: categories
                    .get(&indexed_food.category_id)
                    .ok_or(anyhow!("database was changed during data merging"))?
                    .clone(),
                indexed_food,
            });
        }
        Ok(food)
    }
}

fn from_rows<T: From<Row>>(rows: Vec<Row>) -> Vec<T> {
    rows.into_iter().map(Into::into).collect()
}
