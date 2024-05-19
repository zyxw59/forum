use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Post::Table)
                    .modify_column(
                        ColumnDef::new(Post::Date)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .take(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Post::Table)
                    .modify_column(ColumnDef::new(Post::Date).date_time().not_null())
                    .take(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Date,
}
