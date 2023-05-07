// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{
    auth_from_ctx, db,
    types::{Category, Notification, User},
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

    async fn categories(&self) -> Result<Vec<Category>> {
        self.db.categories().await.map_err(Into::into)
    }
}
