// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use postgres_types::{FromSql, ToSql};
use tokio_postgres::Row;

pub type ID = i32;

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
pub struct Food {
    #[graphql(skip_input)]
    pub id: ID,
    pub title: String,
    pub description: Option<String>,
    pub count: i32,
    pub is_alcohol: bool,
    pub price: f64,
}

impl From<Row> for Food {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            count: row.get("count"),
            is_alcohol: row.get("is_alcohol"),
            price: row.get("price"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "CartInput")]
pub struct Cart {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    pub count: i32,
    #[graphql(skip_input)]
    pub add_time: NaiveDateTime,
}

impl From<Row> for Cart {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            count: row.get("count"),
            add_time: row.get("add_time"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "FavoriteInput")]
pub struct Favorite {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    #[graphql(skip_input)]
    pub add_time: NaiveDateTime,
}

impl From<Row> for Favorite {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            add_time: row.get("add_time"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "OrderInput")]
pub struct Order {
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

impl From<Row> for Order {
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

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "OrderFoodInput")]
pub struct OrderFood {
    #[graphql(skip_input)]
    pub id: ID,
    pub food_id: ID,
    pub count: i32,
}

impl From<Row> for OrderFood {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            food_id: row.get("food_id"),
            count: row.get("count"),
        }
    }
}

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "FeedbackInput")]
pub struct Feedback {
    #[graphql(skip_input)]
    pub id: ID,
    pub order_id: ID,
    /// From 0 to 5.
    pub rating: Option<i8>,
    pub comment: Option<String>,
}

impl From<Row> for Feedback {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            order_id: row.get("order_id"),
            rating: row.get("rating"),
            comment: row.get("comment"),
        }
    }
}
