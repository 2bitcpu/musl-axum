pub mod implement;
pub mod interface;

use implement::content::ContentImplement;
use interface::content::ContentInterface;

#[derive(Clone)]
pub struct Repository {
    pub content: ContentImplement,
}

pub trait RepositoryExtend {
    type ContentRepository: ContentInterface;

    fn content(&self) -> &Self::ContentRepository;
}

impl RepositoryExtend for Repository {
    type ContentRepository = ContentImplement;

    fn content(&self) -> &Self::ContentRepository {
        &self.content
    }
}

impl Repository {
    pub fn new() -> Self {
        Self {
            content: ContentImplement::new(),
        }
    }
}
