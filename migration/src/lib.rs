pub use sea_orm_migration::prelude::*;

mod m20230221_155216_create_customers_table;
mod m20230228_141903_add_timestamps_to_customers;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230221_155216_create_customers_table::Migration),
            Box::new(m20230228_141903_add_timestamps_to_customers::Migration),
        ]
    }
}
