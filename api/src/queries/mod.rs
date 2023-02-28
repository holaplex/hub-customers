#![allow(clippy::unused_async)]

pub mod project;

// // Add your other ones here to create a unified Query object
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(project::Query);
