use sea_orm::Schema;
use sea_orm_migration::prelude::*;

mod entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_database_backend();
        let schema = Schema::new(db);
        manager
            .create_table(schema.create_table_from_entity(entity::forum::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(entity::thread::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(entity::post::Entity))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(entity::forum::Entity)
                    .table(entity::thread::Entity)
                    .table(entity::post::Entity)
                    .take(),
            )
            .await?;
        Ok(())
    }
}
