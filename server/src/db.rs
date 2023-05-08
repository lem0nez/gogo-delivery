// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::{collections::HashMap, env};

use anyhow::anyhow;
use log::error;
use postgres_types::ToSql;
use serde::Deserialize;
use tokio_postgres::{NoTls, Row, ToStatement};

use crate::{sha256, types::*};

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
        self.client
            .query_one(
                include_str!("sql/is_credentials_valid.sql"),
                &[&username, &sha256(password)],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn user_by_name(&self, username: &str) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select_user_by_name.sql"), &[&username])
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

    pub async fn is_user_favorite(&self, username: &str, food_id: ID) -> PostgresResult<bool> {
        self.client
            .query_one(
                include_str!("sql/is_user_favorite.sql"),
                &[&self.user_id_by_name(username).await?, &food_id],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn user_favorites(&self, username: &str) -> anyhow::Result<Vec<Favorite>> {
        let user_id = self.user_id_by_name(username).await?;
        let mut food = self
            .food(
                include_str!("sql/select_user_favorite_food.sql"),
                &[&user_id],
            )
            .await?;
        let indexed_favorites: Vec<IndexedFavorite> = self
            .client
            .query(include_str!("sql/select_user_favorites.sql"), &[&user_id])
            .await
            .map(from_rows)?;

        let mut favorites = Vec::with_capacity(indexed_favorites.capacity());
        for indexed_favorite in indexed_favorites {
            favorites.push(Favorite {
                food: food
                    // We can move a food item because it's
                    // unique per user (constraint 'food_per_user').
                    .remove(&indexed_favorite.food_id)
                    .ok_or(anyhow!("database was changed during data merging"))?,
                indexed_favorite,
            })
        }
        Ok(favorites)
    }

    pub async fn user_cart(
        &self,
        username: &str,
        sort_by: SortCartBy,
        sort_order: SortOrder,
    ) -> anyhow::Result<Vec<CartItem>> {
        let user_id = self.user_id_by_name(username).await?;
        let mut food = self
            .food(
                include_str!("sql/select_food_in_user_cart.sql"),
                &[&user_id],
            )
            .await?;
        let mut indexed_cart: Vec<IndexedCartItem> = self
            .client
            .query(include_str!("sql/select_user_cart.sql"), &[&user_id])
            .await
            .map(from_rows)?;

        indexed_cart.sort_by(|lhs, rhs| sort_by.cmp(lhs, rhs));
        if let SortOrder::Descending = sort_order {
            indexed_cart.reverse();
        }

        let mut cart = Vec::with_capacity(indexed_cart.capacity());
        for indexed_cart_item in indexed_cart {
            cart.push(CartItem {
                food: food
                    // We can move a food item because it's
                    // unique per user (constraint 'food_per_customer').
                    .remove(&indexed_cart_item.food_id)
                    .ok_or(anyhow!("database was changed during data merging"))?,
                indexed_cart_item,
            })
        }
        Ok(cart)
    }

    pub async fn user_orders(&self, username: &str) -> anyhow::Result<Vec<Order>> {
        let customer = self.user_by_name(username).await?;
        let indexed_orders: Vec<IndexedOrder> = self
            .client
            .query(include_str!("sql/select_user_orders.sql"), &[&customer.id])
            .await
            .map(from_rows)?;

        let mut orders = Vec::with_capacity(indexed_orders.capacity());
        for indexed_order in indexed_orders {
            orders.push(Order {
                customer: customer.clone(),
                address: self.address_by_id(indexed_order.address_id).await?,
                rider: match indexed_order.rider_id {
                    Some(id) => Some(self.user_by_id(id).await?),
                    None => None,
                },
                items: self.order_items(indexed_order.id).await?,
                feedback: self.order_feedback(indexed_order.id).await?,
                indexed_order,
            })
        }
        Ok(orders)
    }

    async fn user_by_id(&self, id: ID) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select_user_by_id.sql"), &[&id])
            .await
            .map(Into::into)
    }

    async fn user_id_by_name(&self, username: &str) -> PostgresResult<ID> {
        self.user_by_name(username).await.map(|user| user.id)
    }

    async fn address_by_id(&self, id: ID) -> PostgresResult<Address> {
        self.client
            .query_one(include_str!("sql/select_address_by_id.sql"), &[&id])
            .await
            .map(Into::into)
    }

    async fn food<T: ?Sized + ToStatement>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> anyhow::Result<HashMap<ID, Food>> {
        let categories: HashMap<_, _> = self
            .categories()
            .await?
            .into_iter()
            .map(|category| (category.id, category))
            .collect();
        let indexed_food: Vec<IndexedFood> =
            self.client.query(statement, params).await.map(from_rows)?;

        let mut food = HashMap::with_capacity(indexed_food.capacity());
        // Using loop instead of closure because we must be able to propage an error.
        for indexed_food in indexed_food {
            let category = categories
                .get(&indexed_food.category_id)
                .ok_or(anyhow!("database was changed during data merging"))?
                .clone();
            food.insert(
                indexed_food.id,
                Food {
                    category,
                    indexed_food,
                },
            );
        }
        Ok(food)
    }

    async fn order_items(&self, order_id: ID) -> anyhow::Result<Vec<OrderItem>> {
        let mut food = self
            .food(include_str!("sql/select_order_food.sql"), &[&order_id])
            .await?;
        let indexed_items: Vec<IndexedOrderItem> = self
            .client
            .query(include_str!("sql/select_order_items.sql"), &[&order_id])
            .await
            .map(from_rows)?;

        let mut items = Vec::with_capacity(indexed_items.capacity());
        for indexed_item in indexed_items {
            items.push(OrderItem {
                food: food
                    // We can move a food item because it's
                    // unique per order (constraint 'food_per_order').
                    .remove(&indexed_item.food_id)
                    .ok_or(anyhow!("database was changed during data merging"))?,
                indexed_item,
            })
        }
        Ok(items)
    }

    async fn order_feedback(&self, order_id: ID) -> PostgresResult<Option<Feedback>> {
        self.client
            .query_opt(include_str!("sql/select_order_feedback.sql"), &[&order_id])
            .await
            .map(|row| row.map(Into::into))
    }
}

fn from_rows<T: From<Row>>(rows: Vec<Row>) -> Vec<T> {
    rows.into_iter().map(Into::into).collect()
}
