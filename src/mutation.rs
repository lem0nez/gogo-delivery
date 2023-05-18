// Copyright © 2023 Nikita Dudko. All rights reserved.
// Contacts: <nikita.dudko.95@gmail.com>
// Licensed under the MIT License.

use std::{
    io::{self, Read},
    sync::Arc,
};

use async_graphql::{Context, Object, Result, Upload};
use log::info;

use crate::{auth_from_ctx, db, types::*};

pub struct MutationRoot {
    db: Arc<db::Client>,
}

impl MutationRoot {
    pub fn new(db: Arc<db::Client>) -> Self {
        Self { db }
    }
}

impl MutationRoot {
    async fn current_user(&self, ctx: &Context<'_>) -> Result<User> {
        self.db
            .user_by_name(auth_from_ctx(ctx).user_id())
            .await
            .map_err(Into::into)
    }
}

#[Object]
impl MutationRoot {
    async fn set_user_role(
        &self,
        ctx: &Context<'_>,
        username: String,
        role: UserRole,
    ) -> Result<bool> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        if current_user.username == username {
            return Err("you cannot change role for yourself".into());
        }
        self.db
            .set_user_role(&username, role)
            .await
            .map(|result| {
                if result {
                    info!(
                        "Manager \"{}\" set new role for user \"{username}\"",
                        current_user.username
                    );
                }
                result
            })
            .map_err(Into::into)
    }

    async fn send_direct_notification(
        &self,
        ctx: &Context<'_>,
        target_user_id: ID,
        notification: Notification,
    ) -> Result<ID> {
        let current_user = self.current_user(ctx).await?;
        if let UserRole::Customer = current_user.role {
            return Err("access denied".into());
        }
        self.db
            .add_user_notification(target_user_id, &notification)
            .await
            .map(|id| {
                info!(
                    "User \"{}\" sent direct notification to user with ID {target_user_id}",
                    current_user.username
                );
                id
            })
            .map_err(Into::into)
    }

    async fn broadcast_notification(
        &self,
        ctx: &Context<'_>,
        target_users_role: UserRole,
        notification: Notification,
    ) -> Result<Vec<ID>> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db
            .add_notifications(target_users_role, notification)
            .await
            .map(|ids| {
                info!(
                    "Manager \"{}\" broadcasted a notification",
                    current_user.username
                );
                ids
            })
            .map_err(Into::into)
    }

    async fn add_user_address(&self, ctx: &Context<'_>, address: Address) -> Result<ID> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .add_user_address(username, address)
            .await
            .map(|id| {
                info!("User \"{username}\" added new address with ID {id}");
                id
            })
            .map_err(Into::into)
    }

    async fn delete_user_address(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .delete_user_address(username, id)
            .await
            .map(|result| {
                if result {
                    info!("User \"{username}\" deleted address with ID {id}");
                }
                result
            })
            .map_err(Into::into)
    }

    async fn add_category(
        &self,
        ctx: &Context<'_>,
        category: Category,
        preview: Option<Upload>,
    ) -> Result<ID> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db
            .add_category(&category, read_preview(ctx, preview)?)
            .await
            .map(|id| {
                info!(
                    "Manager \"{}\" added new category \"{}\"",
                    current_user.username, category.title
                );
                id
            })
            .map_err(Into::into)
    }

    async fn delete_category(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db
            .delete_category(id)
            .await
            .map(|result| {
                if result {
                    info!(
                        "Manager \"{}\" deleted category with ID {id}",
                        current_user.username
                    );
                }
                result
            })
            .map_err(Into::into)
    }

    async fn add_food(
        &self,
        ctx: &Context<'_>,
        food: IndexedFood,
        preview: Option<Upload>,
    ) -> Result<ID> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db
            .add_food(&food, read_preview(ctx, preview)?)
            .await
            .map(|id| {
                info!(
                    "Manager \"{}\" added new food \"{}\"",
                    current_user.username, food.title
                );
                id
            })
            .map_err(Into::into)
    }

    async fn delete_food(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let current_user = self.current_user(ctx).await?;
        if current_user.role != UserRole::Manager {
            return Err("access denied".into());
        }
        self.db
            .delete_food(id)
            .await
            .map(|result| {
                if result {
                    info!(
                        "Manager \"{}\" deleted food with ID {id}",
                        current_user.username
                    );
                }
                result
            })
            .map_err(Into::into)
    }

    async fn add_user_favorite(&self, ctx: &Context<'_>, favorite: IndexedFavorite) -> Result<ID> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .add_user_favorite(username, &favorite)
            .await
            .map(|id| {
                info!(
                    "User \"{username}\" added food with ID {} to favorites",
                    favorite.food_id
                );
                id
            })
            .map_err(Into::into)
    }

    async fn delete_user_favorite(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .delete_user_favorite(username, id)
            .await
            .map(|result| {
                if result {
                    info!("User \"{username}\" deleted favorite with ID {id}");
                }
                result
            })
            .map_err(Into::into)
    }

    async fn add_user_cart_item(&self, ctx: &Context<'_>, item: IndexedCartItem) -> Result<ID> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .add_user_cart_item(username, &item)
            .await
            .map(|id| {
                info!(
                    "User \"{username}\" added food with ID {} into the cart",
                    item.food_id
                );
                id
            })
            .map_err(Into::into)
    }

    async fn delete_user_cart_item(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let username = auth_from_ctx(ctx).user_id();
        self.db
            .delete_user_cart_item(username, id)
            .await
            .map(|result| {
                if result {
                    info!("User \"{username}\" deleted cart item with ID {id}");
                }
                result
            })
            .map_err(Into::into)
    }
}

fn read_preview(ctx: &Context<'_>, preview: Option<Upload>) -> io::Result<Option<Vec<u8>>> {
    if preview.is_none() {
        return Ok(None);
    }
    let mut buf = Vec::new();
    let mut file = preview.unwrap().value(ctx)?.content;
    file.read_to_end(&mut buf)?;
    Ok(Some(buf))
}
