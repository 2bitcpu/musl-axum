use crate::model::entity::content::ContentEntity;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentDto {
    pub id: i64,
    pub publish_at: Option<DateTime<Utc>>,
    pub draft: bool,
    pub title: String,
    pub body: String,
}

#[allow(dead_code)]
impl ContentDto {
    pub fn to_entity(&self) -> ContentEntity {
        ContentEntity {
            id: self.id,
            publish_at: self.publish_at,
            draft: self.draft,
            title: self.title.clone(),
            body: self.body.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn from_entity(entity: ContentEntity) -> Self {
        Self {
            id: entity.id,
            publish_at: entity.publish_at,
            draft: entity.draft,
            title: entity.title,
            body: entity.body,
        }
    }
}
