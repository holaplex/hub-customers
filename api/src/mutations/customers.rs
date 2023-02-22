use async_graphql::{self, Context, InputObject, Object, Result, SimpleObject};
use hub_core::producer::Producer;
use sea_orm::{prelude::*, Set};

use crate::{
    entities::{customers, prelude::Customer},
    proto,
    proto::{customer_events::Event, CustomerEventKey, CustomerEvents},
    AppContext,
};

#[derive(Default)]
pub struct Mutation;

#[Object(name = "CustomerMutation")]
impl Mutation {
    /// Res
    ///
    /// # Errors
    /// This function fails if ...
    pub async fn create_customer(
        &self,
        ctx: &Context<'_>,
        input: CreateCustomerInput,
    ) -> Result<CreateCustomerPayload> {
        let AppContext { db, .. } = ctx.data::<AppContext>()?;
        let producer = ctx.data::<Producer<CustomerEvents>>()?;

        let am = customers::ActiveModel {
            project_id: Set(input.project),
            ..Default::default()
        };

        let res = am.insert(db.get()).await?;

        let event = CustomerEvents {
            event: Some(Event::Created(proto::Customer {
                project_id: res.project_id.to_string(),
            })),
        };

        let key = CustomerEventKey {
            id: res.id.to_string(),
        };

        producer.send(Some(&event), Some(&key)).await?;

        Ok(CreateCustomerPayload { customer: res })
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct CreateCustomerInput {
    project: Uuid,
}

#[derive(Debug, Clone, SimpleObject)]
pub struct CreateCustomerPayload {
    customer: Customer,
}
