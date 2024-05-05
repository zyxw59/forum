use sea_orm_migration::prelude::*;

mod entity;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "
CREATE TABLE forum (
    id bigint NOT NULL PRIMARY KEY,
    name character varying(255) NOT NULL,
    parent bigint REFERENCES forum(id)
);
            ").await?;
        db.execute_unprepared(
            "
CREATE TABLE thread (
    id bigint NOT NULL PRIMARY KEY,
    title character varying(255) NOT NULL,
    forum bigint NOT NULL REFERENCES forum(id)
);
            ").await?;
        db.execute_unprepared(
            "
CREATE TABLE post (
    id bigint NOT NULL PRIMARY KEY,
    text text NOT NULL,
    date timestamp without time zone NOT NULL,
    thread bigint NOT NULL REFERENCES thread(id)
);
            ").await?;
            Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
