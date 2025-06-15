pub mod content;

use crate::common::types::DbPool;
use crate::repository::{Repository, RepositoryExtend};
use crate::use_case::content::ContentUseCase;

use std::sync::Arc;

#[derive(Clone)]
pub struct Module {
    pub content: ContentUseCase<Repository>,
}

pub trait ModuleExtend {
    type RepositoryModule: RepositoryExtend;

    fn content(&self) -> &ContentUseCase<Self::RepositoryModule>;
}

impl ModuleExtend for Module {
    type RepositoryModule = Repository;

    fn content(&self) -> &ContentUseCase<Self::RepositoryModule> {
        &self.content
    }
}

impl Module {
    pub fn new(pool: DbPool) -> Self {
        let repo = Arc::new(Repository::new());
        let content = ContentUseCase::new(pool, repo);
        Self { content: content }
    }
}
