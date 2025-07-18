use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            manager,
            "o_auth2_sessions",
            &[
                ("id", ColType::PkAuto),
                ("session_id", ColType::String),
                ("expires_at", ColType::TimestampWithTimeZone),
                ("user_id", ColType::Integer),
            ],
            &[]
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("o_auth2_sessions")).to_owned())
            .await
    }
}