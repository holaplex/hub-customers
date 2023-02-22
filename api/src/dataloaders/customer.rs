use std::collections::HashMap;

use async_graphql::{dataloader::Loader as DataLoader, FieldError, Result};
use poem::async_trait;
use sea_orm::prelude::*;

use crate::{
    db::Connection,
    entities::customers::{Column, Entity, Model as Customer},
};

#[derive(Debug, Clone)]
pub struct Loader {
    pub db: Connection,
}

impl Loader {
    #[must_use]
    pub fn new(db: Connection) -> Self {
        Self { db }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(pub Uuid);

#[async_trait]
impl DataLoader<ProjectId> for Loader {
    type Error = FieldError;
    type Value = Vec<Customer>;

    async fn load(
        &self,
        keys: &[ProjectId],
    ) -> Result<HashMap<ProjectId, Self::Value>, Self::Error> {
        let customers = Entity::find()
            .filter(Column::ProjectId.is_in(keys.iter().map(|p| p.0)))
            .all(self.db.get())
            .await?;

        let mut hashmap = HashMap::new();

        for c in customers {
            hashmap
                .entry(ProjectId(c.project_id))
                .or_insert(Vec::new())
                .push(c);
        }

        Ok(hashmap)
    }
}

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Customer;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let customers = Entity::find()
            .filter(Column::Id.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(customers.into_iter().map(|c| (c.id, c.into())).collect())
    }
}
