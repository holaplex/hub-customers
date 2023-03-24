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
    /// This mutation creates a customer record and a corresponding treasury that holds custodial wallets on behalf of a user. The treasury serves as a way to group the customer's wallets together. This makes it easier to manage wallets and associated assets for the user within a specific project. The customer and treasury are associated with the specified project ID. The response includes the newly created customer record. If there is an error connecting to the database or unable to emit a customer created event, the mutation will fail and an error will be returned.
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

/// This input object is used for creating a customer and associated treasury for holding custodial wallets on behalf of the user.
#[derive(Debug, Clone, InputObject)]
pub struct CreateCustomerInput {
    /// The unique identifier of the project to which the customer is associated.
    project: Uuid,
}

/// This response represents the payload returned after successfully creating a new `customer` record. It contains a single field customer which is a `Customer` object representing the newly created customer record.
#[derive(Debug, Clone, SimpleObject)]
pub struct CreateCustomerPayload {
    /// The customer record created by the create customer mutation.
    customer: Customer,
}
