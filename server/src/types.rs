// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::cmp::Ordering;

use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use postgres_types::{FromSql, ToSql};
use rust_decimal::Decimal;
use tokio_postgres::Row;

pub type ID = i32;

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum SortOrder {
    Ascending,
    Descending,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromSql, ToSql, Enum)]
pub enum UserRole {
    Customer,
    Manager,
    Rider,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "UserInput")]
pub struct User {
    #[graphql(skip_input)]
    pub id: ID,
    pub username: String,
    /// SHA256-encrypted string.
    #[graphql(skip_output)]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[graphql(skip_output)]
    pub birth_date: NaiveDate,
    pub role: UserRole,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            username: row.get("username"),
            password: row.get("password"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            birth_date: row.get("birth_date"),
            role: row.get("role"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum SortUsersBy {
    Username,
    FirstName,
    LastName,
}

impl SortUsersBy {
    pub fn cmp(&self, lhs: &User, rhs: &User) -> Ordering {
        match self {
            Self::Username => lhs.username.cmp(&rhs.username),
            Self::FirstName => lhs.first_name.cmp(&rhs.first_name),
            Self::LastName => lhs.last_name.cmp(&rhs.last_name),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "NotificationInput")]
pub struct Notification {
    #[graphql(skip_input)]
    pub id: ID,
    #[graphql(skip_input)]
    pub sent_time: NaiveDateTime,
    pub title: String,
    pub description: Option<String>,
}

impl From<Row> for Notification {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            sent_time: row.get("sent_time"),
            title: row.get("title"),
            description: row.get("description"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "AddressInput")]
pub struct Address {
    #[graphql(skip_input)]
    pub id: ID,
    pub locality: String,
    pub street: String,
    pub house: i32,
    pub corps: Option<String>,
    pub apartment: Option<String>,
}

impl From<Row> for Address {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            locality: row.get("locality"),
            street: row.get("street"),
            house: row.get("house"),
            corps: row.get("corps"),
            apartment: row.get("apartment"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CategoryInput")]
pub struct Category {
    #[graphql(skip_input)]
    pub id: ID,
    pub title: String,
    pub description: Option<String>,
}

impl From<Row> for Category {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "FoodInput")]
pub struct IndexedFood {
    #[graphql(skip_input)]
    pub id: ID,
    pub title: String,
    pub description: Option<String>,
    pub category_id: ID,
    pub count: i32,
    pub is_alcohol: bool,
    pub price: Decimal,
}

impl From<Row> for IndexedFood {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            category_id: row.get("category_id"),
            count: row.get("count"),
            is_alcohol: row.get("is_alcohol"),
            price: row.get("price"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum SortFoodBy {
    Title,
    Count,
    Price,
}

impl SortFoodBy {
    pub fn cmp(&self, lhs: &IndexedFood, rhs: &IndexedFood) -> Ordering {
        match self {
            Self::Title => lhs.title.cmp(&rhs.title),
            Self::Count => lhs.count.cmp(&rhs.count),
            Self::Price => lhs.price.partial_cmp(&rhs.price).unwrap_or(Ordering::Equal),
        }
    }
}

#[derive(SimpleObject)]
pub struct Food {
    pub category: Category,
    pub food_data: IndexedFood,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CartInput")]
pub struct IndexedCart {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    pub count: i32,
    #[graphql(skip_input)]
    pub add_time: NaiveDateTime,
}

impl From<Row> for IndexedCart {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            count: row.get("count"),
            add_time: row.get("add_time"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Enum)]
pub enum SortCartBy {
    Count,
    AddTime,
}

impl SortCartBy {
    pub fn cmp(&self, lhs: &IndexedCart, rhs: &IndexedCart) -> Ordering {
        match self {
            Self::Count => lhs.count.cmp(&rhs.count),
            Self::AddTime => lhs.add_time.cmp(&rhs.add_time),
        }
    }
}

#[derive(SimpleObject)]
pub struct Cart {
    pub food: Food,
    pub cart_data: IndexedCart,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "FavoriteInput")]
pub struct IndexedFavorite {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    #[graphql(skip_input)]
    pub add_time: NaiveDateTime,
}

impl From<Row> for IndexedFavorite {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            add_time: row.get("add_time"),
        }
    }
}

#[derive(SimpleObject)]
pub struct Favorite {
    pub food: Food,
    pub favorite_data: IndexedFavorite,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "OrderInput")]
pub struct IndexedOrder {
    #[graphql(skip_input)]
    pub id: ID,
    pub customer_id: ID,
    pub address_id: ID,
    #[graphql(skip_input)]
    pub create_time: NaiveDateTime,
    #[graphql(skip_input)]
    pub rider_id: Option<ID>,
    #[graphql(skip_input)]
    pub completed_time: Option<NaiveDateTime>,
}

impl From<Row> for IndexedOrder {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            customer_id: row.get("customer_id"),
            address_id: row.get("address_id"),
            create_time: row.get("create_time"),
            rider_id: row.get("rider_id"),
            completed_time: row.get("completed_time"),
        }
    }
}

#[derive(SimpleObject)]
pub struct Order {
    pub customer: User,
    pub address: Address,
    pub rider: User,
    pub food: Vec<OrderItem>,
    pub order_data: IndexedOrder,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "OrderItemInput")]
pub struct IndexedOrderItem {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    pub count: i32,
}

impl From<Row> for IndexedOrderItem {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            count: row.get("count"),
        }
    }
}

#[derive(SimpleObject)]
pub struct OrderItem {
    pub food: Food,
    pub order_item_data: IndexedOrderItem,
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "FeedbackInput")]
pub struct IndexedFeedback {
    #[graphql(skip_input)]
    pub id: ID,
    pub order_id: ID,
    /// From 0 to 5.
    pub rating: Option<i8>,
    pub comment: Option<String>,
}

impl From<Row> for IndexedFeedback {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            order_id: row.get("order_id"),
            rating: row.get("rating"),
            comment: row.get("comment"),
        }
    }
}

#[derive(SimpleObject)]
pub struct Feedback {
    pub order: Order,
    pub feedback_data: IndexedFeedback,
}
