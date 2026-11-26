use std::{
    collections::{HashMap, hash_map::Entry},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use async_trait::async_trait;

use crate::{
    domain::{color::Color, tag::Tag},
    repositories::{repository_error::RepositoryError, tags_repository::TagsRepository},
};

pub struct InMemoryTagsRepository {
    tags: RwLock<HashMap<String, Tag>>,
}

impl InMemoryTagsRepository {
    const REPO_PARSE_MESSAGE: &str = "hardcoded version of repo";

    pub fn new() -> Self {
        let beach = Tag::new(
            "beach".to_string(),
            Color::parse("#0000ff".to_string()).expect(Self::REPO_PARSE_MESSAGE),
        );
        let climbing = Tag::new(
            "climbing".to_string(),
            Color::parse("#ff0000".to_string()).expect(Self::REPO_PARSE_MESSAGE),
        );

        Self {
            tags: RwLock::new(HashMap::from([
                (beach.label.clone(), beach),
                (climbing.label.clone(), climbing),
            ])),
        }
    }

    fn read_tags(&self) -> Result<RwLockReadGuard<'_, HashMap<String, Tag>>, RepositoryError> {
        self.tags
            .read()
            .map_err(|e| RepositoryError::UnknownState(e.to_string()))
    }

    fn write_tags(&self) -> Result<RwLockWriteGuard<'_, HashMap<String, Tag>>, RepositoryError> {
        self.tags
            .write()
            .map_err(|e| RepositoryError::UnknownState(e.to_string()))
    }
}

#[async_trait]
impl TagsRepository for InMemoryTagsRepository {
    async fn all(&self) -> Result<Vec<Tag>, RepositoryError> {
        Ok(self.read_tags()?.values().cloned().collect())
    }

    async fn create(&self, tag: Tag) -> Result<Tag, RepositoryError> {
        match self.write_tags()?.entry(tag.label.clone()) {
            Entry::Occupied(_) => Err(RepositoryError::AlreadyExists(tag.label.to_string())),
            Entry::Vacant(slot) => Ok(slot.insert(tag).clone()),
        }
    }

    async fn find_many_by_labels(&self, labels: &[String]) -> Result<Vec<Tag>, RepositoryError> {
        Ok(self
            .read_tags()?
            .values()
            .filter(|tag| labels.contains(&tag.label))
            .cloned()
            .collect())
    }
}
