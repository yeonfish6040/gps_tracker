pub use sea_orm_migration::prelude::*;

#[path = "m20260316_000001_create_devices_table.rs"]
mod m20260316_000001_create_devices_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            self::m20260316_000001_create_devices_table::Migration,
        )]
    }
}
