use loco_rs::schema::{create_table, ColType};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "entries",
            &[
                ("id", ColType::PkAuto),
                ("date", ColType::Date),
                ("start_time", ColType::DateTimeNull),
                ("end_time", ColType::DateTimeNull),
                ("room_id", ColType::Integer),
                ("timestamp", ColType::DateTime),
                ("create_by", ColType::String),
                ("modified_by", ColType::String),
                ("name", ColType::String),
                ("registrant_limit", ColType::IntegerNull),
                ("description", ColType::TextNull),
            ],
            &[],
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ix_entries_room_id")
                .table(Alias::new("entries"))
                .col(Alias::new("room_id"))
                .to_owned(),
        )
            .await?;
        m.create_foreign_key(
            ForeignKey::create()
                .name("fk_entry_room")
                .from(Alias::new("entries"), Alias::new("room_id"))
                .to(Alias::new("rooms"), Alias::new("id"))
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Restrict)
                .to_owned(),
        )
            .await?;
        
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Alias::new("entries")).to_owned())
            .await?;
        Ok(())
    }
}

