use crate::common::types::{BoxError, DbPool};
use crate::model::dto::content::ContentDto;
use crate::repository::RepositoryExtend;
use crate::repository::interface::content::ContentInterface;

use derive_new::new;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct ContentUseCase<R: RepositoryExtend> {
    pool: DbPool,
    repository: Arc<R>,
}

impl<R: RepositoryExtend> ContentUseCase<R> {
    pub async fn create(&self, dto: ContentDto) -> Result<ContentDto, BoxError> {
        let mut executor = self.pool.begin().await?;
        let result = self
            .repository
            .content()
            .insert(&mut *executor, dto.to_entity())
            .await?;
        executor.commit().await?;
        Ok(ContentDto::from_entity(result))
    }

    pub async fn find(&self, id: i64) -> Result<ContentDto, BoxError> {
        let mut executor = self.pool.acquire().await?;
        let result = self
            .repository
            .content()
            .select(&mut *executor, id)
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;
        Ok(ContentDto::from_entity(result))
    }

    pub async fn edit(&self, dto: ContentDto) -> Result<ContentDto, BoxError> {
        let mut executor = self.pool.begin().await?;
        let result = self
            .repository
            .content()
            .update(&mut *executor, dto.to_entity())
            .await?
            .ok_or_else(|| sqlx::Error::RowNotFound)?;
        executor.commit().await?;
        Ok(ContentDto::from_entity(result))
    }

    pub async fn remove(&self, id: i64) -> Result<u64, BoxError> {
        let mut executor = self.pool.begin().await?;
        let result = self.repository.content().delete(&mut *executor, id).await?;
        if result > 0 {
            executor.commit().await?;
        } else {
            return Err(sqlx::Error::RowNotFound.into());
        }
        Ok(result)
    }
}
