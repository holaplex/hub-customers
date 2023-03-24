use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use hub_core::uuid::Uuid;

use crate::{entities::prelude::Customer, AppContext};

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex, concrete(name = "Project", params()))]
pub struct Project {
    #[graphql(external)]
    pub id: Uuid,
}

#[ComplexObject]
impl Project {
    /// Retrieve a customer record associated with the project, using its ID.
    async fn customer(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Customer>> {
        let AppContext {
            customer_loader, ..
        } = ctx.data::<AppContext>()?;
        let customer = customer_loader.load_one(id).await?;

        match customer.clone() {
            Some(c) if c.project_id == self.id => Ok(customer),
            Some(_) | None => Ok(None),
        }
    }

    /// Retrieve all customer records associated with a given project.
    async fn customers(&self, ctx: &Context<'_>) -> Result<Option<Vec<Customer>>> {
        let AppContext {
            project_customers_loader,
            ..
        } = ctx.data::<AppContext>()?;
        project_customers_loader.load_one(self.id).await
    }
}
