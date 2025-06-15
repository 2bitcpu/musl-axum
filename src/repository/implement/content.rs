use crate::common::types::{BoxError, DbExecutor};
use crate::model::entity::content::ContentEntity;
use crate::repository::interface::content::ContentInterface;

use async_trait::async_trait;

#[derive(Clone)]
pub struct ContentImplement;

impl ContentImplement {
    pub fn new() -> Self {
        Self
    }
}

#[rustfmt::skip]
#[async_trait]
impl ContentInterface for ContentImplement {
    async fn insert(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<ContentEntity, BoxError> {
    let sql = "INSERT INTO content (publish_at,draft,title,body) VALUES ($1,$2,$3,$4) RETURNING *";
    Ok(sqlx::query_as::<_, ContentEntity>(sql)
        .bind(entity.publish_at)
        .bind(entity.draft)
        .bind(entity.title)
        .bind(entity.body)
        .fetch_one(&mut *executor)
        .await?)    
    }

    async fn update(&self, executor: &mut DbExecutor, entity: ContentEntity) -> Result<Option<ContentEntity>, BoxError> {
        let sql = "UPDATE content SET publish_at=$2,draft=$3,title=$4,body=$5,updated_at=CURRENT_TIMESTAMP WHERE id=$1 RETURNING *";
        Ok(sqlx::query_as::<_, ContentEntity>(sql)
            .bind(entity.id)
            .bind(entity.publish_at)
            .bind(entity.draft)
            .bind(entity.title)
            .bind(entity.body)
            .fetch_optional(&mut *executor)
            .await?)
    }

    async fn select(&self, executor: &mut DbExecutor, id: i64) -> Result<Option<ContentEntity>, BoxError> {
        let sql = "SELECT * FROM content WHERE id=$1";
        Ok(sqlx::query_as::<_, ContentEntity>(sql)
            .bind(id)
            .fetch_optional(&mut *executor)
            .await?)
    }

    async fn delete(&self, executor: &mut DbExecutor, id: i64) -> Result<u64, BoxError> {
        let sql = "DELETE FROM content WHERE id=$1 RETURNING *";
        Ok(sqlx::query(sql)
            .bind(id)
            .execute(&mut *executor)
            .await?
            .rows_affected())
    }    
}
