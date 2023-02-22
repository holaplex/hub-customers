pub mod customers;

// // Add your other ones here to create a unified Query object
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(customers::Query);
