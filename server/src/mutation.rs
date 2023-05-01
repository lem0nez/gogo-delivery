use std::sync::Arc;

use async_graphql::Object;

use crate::db;

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
    async fn _remove_me(&self) -> bool {
        todo!()
    }
}
