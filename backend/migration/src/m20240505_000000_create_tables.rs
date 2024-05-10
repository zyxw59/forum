use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Forum::Table)
                    .col(
                        ColumnDef::new(Forum::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Forum::Title).string().not_null())
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Thread::Table)
                    .col(
                        ColumnDef::new(Thread::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Thread::Forum).integer().not_null())
                    .col(ColumnDef::new(Thread::Title).string().not_null())
                    .take(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Forum::Table, Forum::Id)
                    .to(Thread::Table, Thread::Id)
                    .take(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .col(ColumnDef::new(Post::Thread).big_integer().not_null())
                    .col(ColumnDef::new(Post::Id).small_integer().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::Date).date_time().not_null())
                    .primary_key(Index::create().col(Post::Thread).col(Post::Id))
                    .take(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(Post::Table, Post::Thread)
                    .to(Thread::Table, Thread::Id)
                    .take(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Forum::Table)
                    .table(Thread::Table)
                    .table(Post::Table)
                    .take(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Forum {
    Table,
    Id,
    Title,
}

#[derive(DeriveIden)]
enum Thread {
    Table,
    Id,
    Forum,
    Title,
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Thread,
    Text,
    Date,
}
