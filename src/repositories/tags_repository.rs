use async_trait::async_trait;

use crate::{domain::tag::Tag, repositories::repository_error::RepositoryError};

#[async_trait]
pub trait TagsRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Tag>, RepositoryError>;
    async fn create(&self, tag: Tag) -> Result<Tag, RepositoryError>;
    async fn find_many_by_labels(&self, labels: &[String]) -> Result<Vec<Tag>, RepositoryError>;
}
