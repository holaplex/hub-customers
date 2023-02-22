use async_graphql::{ComplexObject, Context, Object, Result, SimpleObject};
use sea_orm::prelude::*;

use crate::{dataloaders::customer::ProjectId, entities::prelude::Customer, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "CustomersQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_project_by_id(
        &self,
        _ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Project> {
        Ok(Project { id })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex, concrete(name = "Project", params()))]
pub struct Project {
    #[graphql(external)]
    pub id: Uuid,
}

#[ComplexObject]
impl Project {
    async fn customer(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Customer>> {
        let AppContext {
            customers_loader, ..
        } = ctx.data::<AppContext>()?;
        let customer = customers_loader.load_one(id).await?;

        match customer.clone() {
            Some(c) if c.project_id == self.id => Ok(customer),
            Some(_) | None => Ok(None),
        }
    }

    async fn customers(&self, ctx: &Context<'_>) -> Result<Option<Vec<Customer>>> {
        let AppContext {
            customers_loader, ..
        } = ctx.data::<AppContext>()?;
        customers_loader.load_one(ProjectId(self.id)).await
    }
}
