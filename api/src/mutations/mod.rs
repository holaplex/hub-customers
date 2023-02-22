#![allow(clippy::too_many_lines)]

pub mod customers;

// // Add your other ones here to create a unified Mutation object
// // e.x. Mutation(OrganizationMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(customers::Mutation);
