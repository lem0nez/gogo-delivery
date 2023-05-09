// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::{collections::HashMap, env};

use anyhow::anyhow;
use log::error;
use postgres_types::ToSql;
use serde::Deserialize;
use tokio_postgres::{NoTls, Row};

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
        self.is_true(
            include_str!("sql/check/credentials_valid.sql"),
            &[&username, &sha256(password)],
        )
        .await
    }

    pub async fn user_by_name(&self, username: &str) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select/user_by_name.sql"), &[&username])
            .await
            .map(Into::into)
    }

    pub async fn users(&self) -> PostgresResult<Vec<User>> {
        self.client
            .query(include_str!("sql/select/users.sql"), &[])
            .await
            .map(from_rows)
    }

    pub async fn add_user(&self, user: User) -> PostgresResult<ID> {
        self.client
            .query_one(
                include_str!("sql/insert/user.sql"),
                &[
                    &user.username,
                    &user.password,
                    &user.first_name,
                    &user.last_name,
                    &user.birth_date,
                ],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn set_user_role(&self, username: &str, role: UserRole) -> PostgresResult<bool> {
        self.client
            .execute(
                include_str!("sql/update/user_role.sql"),
                &[&self.user_id_by_name(username).await?, &role],
            )
            .await
            .map(|modified_rows| modified_rows != 0)
    }

    pub async fn user_notifications(&self, username: &str) -> PostgresResult<Vec<Notification>> {
        self.client
            .query(
                include_str!("sql/select/user_notifications.sql"),
                &[&self.user_id_by_name(username).await?],
            )
            .await
            .map(from_rows)
    }

    pub async fn user_addresses(&self, username: &str) -> PostgresResult<Vec<Address>> {
        self.client
            .query(
                include_str!("sql/select/user_addresses.sql"),
                &[&self.user_id_by_name(username).await?],
            )
            .await
            .map(from_rows)
    }

    pub async fn add_user_address(&self, username: &str, address: Address) -> PostgresResult<ID> {
        self.client
            .query_one(
                include_str!("sql/insert/user_address.sql"),
                &[
                    &self.user_id_by_name(username).await?,
                    &address.locality,
                    &address.street,
                    &address.house,
                    &address.corps,
                    &address.apartment,
                ],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn delete_user_address(&self, username: &str, id: ID) -> PostgresResult<bool> {
        self.client
            .execute(
                include_str!("sql/delete/user_address.sql"),
                &[&self.user_id_by_name(username).await?, &id],
            )
            .await
            .map(|modified_rows| modified_rows != 0)
    }

    pub async fn categories(&self) -> PostgresResult<Vec<Category>> {
        self.client
            .query(include_str!("sql/select/categories.sql"), &[])
            .await
            .map(from_rows)
    }

    pub async fn add_category(
        &self,
        category: &Category,
        preview: Option<Vec<u8>>,
    ) -> PostgresResult<ID> {
        self.client
            .query_one(
                include_str!("sql/insert/category.sql"),
                &[&category.title, &category.description, &preview],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn delete_category(&self, id: ID) -> PostgresResult<bool> {
        self.client
            .execute(include_str!("sql/delete/category.sql"), &[&id])
            .await
            .map(|modified_rows| modified_rows != 0)
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
                include_str!("sql/select/food_in_category.sql"),
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

    pub async fn add_food(
        &self,
        food: &IndexedFood,
        preview: Option<Vec<u8>>,
    ) -> PostgresResult<ID> {
        self.client
            .query_one(
                include_str!("sql/insert/food.sql"),
                &[
                    &food.title,
                    &food.description,
                    &preview,
                    &food.category_id,
                    &food.count,
                    &food.is_alcohol,
                    &food.price,
                ],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn delete_food(&self, id: ID) -> PostgresResult<bool> {
        self.client
            .execute(include_str!("sql/delete/food.sql"), &[&id])
            .await
            .map(|modified_rows| modified_rows != 0)
    }

    pub async fn preview(&self, of: PreviewOf, id: ID) -> PostgresResult<Vec<u8>> {
        self.client
            .query_one(
                match of {
                    PreviewOf::Category => include_str!("sql/select/category_preview.sql"),
                    PreviewOf::Food => include_str!("sql/select/food_preview.sql"),
                },
                &[&id],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn is_user_favorite(&self, username: &str, food_id: ID) -> PostgresResult<bool> {
        self.is_true(
            include_str!("sql/check/user_favorite.sql"),
            &[&self.user_id_by_name(username).await?, &food_id],
        )
        .await
    }

    pub async fn user_favorites(&self, username: &str) -> anyhow::Result<Vec<Favorite>> {
        let user_id = self.user_id_by_name(username).await?;
        let mut food = self
            .query_food(
                include_str!("sql/select/user_favorite_food.sql"),
                &[&user_id],
            )
            .await?;
        let indexed_favorites: Vec<IndexedFavorite> = self
            .client
            .query(include_str!("sql/select/user_favorites.sql"), &[&user_id])
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

    pub async fn add_user_favorite(
        &self,
        username: &str,
        favorite: &IndexedFavorite,
    ) -> PostgresResult<ID> {
        self.client
            .query_one(
                include_str!("sql/insert/user_favorite.sql"),
                &[&self.user_id_by_name(username).await?, &favorite.food_id],
            )
            .await
            .map(|row| row.get(0))
    }

    pub async fn delete_user_favorite(&self, username: &str, id: ID) -> PostgresResult<bool> {
        self.client
            .execute(
                include_str!("sql/delete/user_favorite.sql"),
                &[&self.user_id_by_name(username).await?, &id],
            )
            .await
            .map(|modified_rows| modified_rows != 0)
    }

    pub async fn is_in_user_cart(&self, username: &str, food_id: ID) -> PostgresResult<bool> {
        self.is_true(
            include_str!("sql/check/in_user_cart.sql"),
            &[&self.user_id_by_name(username).await?, &food_id],
        )
        .await
    }

    pub async fn user_cart(
        &self,
        username: &str,
        sort_by: SortCartBy,
        sort_order: SortOrder,
    ) -> anyhow::Result<Vec<CartItem>> {
        let user_id = self.user_id_by_name(username).await?;
        let mut food = self
            .query_food(
                include_str!("sql/select/food_in_user_cart.sql"),
                &[&user_id],
            )
            .await?;
        let mut indexed_cart: Vec<IndexedCartItem> = self
            .client
            .query(include_str!("sql/select/user_cart.sql"), &[&user_id])
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

    pub async fn orders(&self, filter: OrdersFilter) -> anyhow::Result<Vec<Order>> {
        self.query_orders(include_str!("sql/select/orders.sql"), &[], filter)
            .await
    }

    pub async fn user_orders(
        &self,
        username: &str,
        filter: OrdersFilter,
    ) -> anyhow::Result<Vec<Order>> {
        self.query_orders(
            include_str!("sql/select/user_orders.sql"),
            &[&self.user_id_by_name(username).await?],
            filter,
        )
        .await
    }

    async fn user_by_id(&self, id: ID) -> PostgresResult<User> {
        self.client
            .query_one(include_str!("sql/select/user_by_id.sql"), &[&id])
            .await
            .map(Into::into)
    }

    async fn user_id_by_name(&self, username: &str) -> PostgresResult<ID> {
        self.user_by_name(username).await.map(|user| user.id)
    }

    async fn address_by_id(&self, id: ID) -> PostgresResult<Address> {
        self.client
            .query_one(include_str!("sql/select/address_by_id.sql"), &[&id])
            .await
            .map(Into::into)
    }

    async fn query_food(
        &self,
        statement: &str,
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

    async fn query_orders(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
        filter: OrdersFilter,
    ) -> anyhow::Result<Vec<Order>> {
        let indexed_orders: Vec<IndexedOrder> = self
            .client
            .query(statement, params)
            .await
            .map(from_rows)?
            .into_iter()
            .filter(|order| filter.fits(order))
            .collect();

        let mut orders = Vec::with_capacity(indexed_orders.capacity());
        for indexed_order in indexed_orders {
            orders.push(Order {
                customer: self.user_by_id(indexed_order.customer_id).await?,
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

    async fn order_items(&self, order_id: ID) -> anyhow::Result<Vec<OrderItem>> {
        let mut food = self
            .query_food(include_str!("sql/select/order_food.sql"), &[&order_id])
            .await?;
        let indexed_items: Vec<IndexedOrderItem> = self
            .client
            .query(include_str!("sql/select/order_items.sql"), &[&order_id])
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
            .query_opt(include_str!("sql/select/order_feedback.sql"), &[&order_id])
            .await
            .map(|row| row.map(Into::into))
    }

    async fn is_true(
        &self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> PostgresResult<bool> {
        self.client
            .query_one(statement, params)
            .await
            .map(|row| row.get(0))
    }
}

fn from_rows<T: From<Row>>(rows: Vec<Row>) -> Vec<T> {
    rows.into_iter().map(Into::into).collect()
}
