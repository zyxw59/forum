pub use sea_orm_migration::prelude::*;

mod m20240505_000000_create_tables;
mod m20240513_005933_default_date;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240505_000000_create_tables::Migration),
            Box::new(m20240513_005933_default_date::Migration),
        ]
    }
}
