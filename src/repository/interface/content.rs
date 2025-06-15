use crate::common::types::{BoxError, DbExecutor};
use crate::model::entity::content::ContentEntity;

use async_trait::async_trait;

#[rustfmt::skip]
#[async_trait]
pub trait ContentInterface {
    async fn insert(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<ContentEntity, BoxError>;
    async fn update(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<Option<ContentEntity>, BoxError>;
    async fn select(&self, executor: &mut DbExecutor, id: i64) -> Result<Option<ContentEntity>, BoxError>;
    async fn delete(&self, executor: &mut DbExecutor, id: i64) -> Result<u64, BoxError>;
}
