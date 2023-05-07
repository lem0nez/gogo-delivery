// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{
    auth_from_ctx, db,
    types::{Address, Category, Food, FoodOrder, Notification, User, ID},
};

pub struct QueryRoot {
    db: Arc<db::Client>,
}

impl QueryRoot {
    pub fn new(db: Arc<db::Client>) -> Self {
        Self { db }
    }
}

#[Object]
impl QueryRoot {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<User> {
        self.db
            .user(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn notifications(&self, ctx: &Context<'_>) -> Result<Vec<Notification>> {
        self.db
            .notifications(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn addresses(&self, ctx: &Context<'_>) -> Result<Vec<Address>> {
        self.db
            .addresses(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn categories(&self) -> Result<Vec<Category>> {
        self.db.categories().await.map_err(Into::into)
    }

    async fn food(&self, category_id: ID, order_by: FoodOrder) -> Result<Vec<Food>> {
        self.db
            .food(category_id, order_by)
            .await
            .map_err(Into::into)
    }
}
