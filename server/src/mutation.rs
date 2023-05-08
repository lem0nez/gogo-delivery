// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::{auth_from_ctx, db, types::*};

pub struct MutationRoot {
    db: Arc<db::Client>,
}

impl MutationRoot {
    pub fn new(db: Arc<db::Client>) -> Self {
        Self { db }
    }
}

#[Object]
impl MutationRoot {
    /// Returns ID of the new favorite item.
    async fn add_user_favorite(
        &self,
        ctx: &Context<'_>,
        favorite: IndexedFavorite,
    ) -> Result<ID> {
        self.db
            .add_user_favorite(auth_from_ctx(ctx).user_id(), favorite)
            .await
            .map_err(Into::into)
    }
}
