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

#[async_trait]
impl DataLoader<Uuid> for Loader {
    type Error = FieldError;
    type Value = Vec<Customer>;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        let customers = Entity::find()
            .filter(Column::ProjectId.is_in(keys.iter().map(ToOwned::to_owned)))
            .all(self.db.get())
            .await?;

        Ok(customers
            .into_iter()
            .fold(HashMap::new(), |mut acc, customer| {
                acc.entry(customer.project_id).or_insert_with(Vec::new);

                acc.entry(customer.project_id)
                    .and_modify(|customers| customers.push(customer));

                acc
            }))
    }
}
