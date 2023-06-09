// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{auth_from_ctx, db, types::*};

pub struct QueryRoot {
    db: Arc<db::Client>,
}

impl QueryRoot {
    pub fn new(db: Arc<db::Client>) -> Self {
        Self { db }
    }
}

impl QueryRoot {
    async fn current_user_impl(&self, ctx: &Context<'_>) -> Result<User> {
        self.db
            .user_by_name(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }
}

#[Object]
impl QueryRoot {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<User> {
        self.current_user_impl(ctx).await
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        if self.current_user_impl(ctx).await?.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db.users().await.map_err(Into::into)
    }

    async fn user_notifications(&self, ctx: &Context<'_>) -> Result<Vec<Notification>> {
        self.db
            .user_notifications(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn user_addresses(&self, ctx: &Context<'_>) -> Result<Vec<Address>> {
        self.db
            .user_addresses(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn categories(&self) -> Result<Vec<Category>> {
        self.db.categories().await.map_err(Into::into)
    }

    async fn food_in_category(
        &self,
        category_id: ID,
        sort_by: SortFoodBy,
        sort_order: SortOrder,
    ) -> Result<Vec<IndexedFood>> {
        self.db
            .food_in_category(category_id, sort_by, sort_order)
            .await
            .map_err(Into::into)
    }

    async fn is_user_favorite(&self, ctx: &Context<'_>, food_id: ID) -> Result<bool> {
        self.db
            .is_user_favorite(auth_from_ctx(ctx).user_id(), food_id)
            .await
            .map_err(Into::into)
    }

    async fn user_favorites(&self, ctx: &Context<'_>) -> Result<Vec<Favorite>> {
        self.db
            .user_favorites(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }

    async fn is_in_user_cart(&self, ctx: &Context<'_>, food_id: ID) -> Result<bool> {
        self.db
            .is_in_user_cart(auth_from_ctx(ctx).user_id(), food_id)
            .await
            .map_err(Into::into)
    }

    async fn user_cart(
        &self,
        ctx: &Context<'_>,
        sort_by: SortCartBy,
        sort_order: SortOrder,
    ) -> Result<Cart> {
        self.db
            .user_cart(auth_from_ctx(ctx).user_id(), sort_by, sort_order)
            .await
            .map_err(Into::into)
    }

    async fn orders(&self, ctx: &Context<'_>, filter: OrdersFilter) -> Result<Vec<Order>> {
        if let UserRole::Customer = self.current_user_impl(ctx).await?.role {
            return Err("access denied".into());
        }
        self.db.orders(filter).await.map_err(Into::into)
    }

    async fn user_orders(&self, ctx: &Context<'_>, filter: OrdersFilter) -> Result<Vec<Order>> {
        self.db
            .user_orders(auth_from_ctx(ctx).user_id(), filter)
            .await
            .map_err(Into::into)
    }
}
