// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use async_graphql::{Context, Object};

use crate::{auth_from_ctx, db, types::User};

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
    async fn current_user(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        self.db
            .get_user(auth_from_ctx(ctx).user_id())
            .await?
            .ok_or("no such user in database".into())
    }
}
