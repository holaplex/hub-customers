use async_graphql::{Context, Object, Result};
use sea_orm::prelude::*;

use crate::{entities::customers::Model, AppContext};

#[derive(Debug, Clone, Copy, Default)]
pub struct Query;

#[Object(name = "CustomerQuery")]
impl Query {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    #[graphql(entity)]
    async fn find_customer_by_id(
        &self,
        ctx: &Context<'_>,
        #[graphql(key)] id: Uuid,
    ) -> Result<Option<Model>> {
        let AppContext {
            customer_loader, ..
        } = ctx.data::<AppContext>()?;

        customer_loader.load_one(id).await
    }
}
