use loco_rs::schema::{create_table, ColType};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "participants",
            &[
                ("id", ColType::PkAuto),
                ("entry_id", ColType::Integer),
                ("workspace_id", ColType::Integer),
                ("username", ColType::StringNull),
                ("create_by", ColType::StringNull),
                ("registered", ColType::DateTime),
            ],
            &[],
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ix_participants_entry_id")
                .table(Alias::new("participants"))
                .col(Alias::new("entry_id"))
                .to_owned(),
        )
            .await?;
        m.create_foreign_key(
            ForeignKey::create()
                .name("fk_participants_entry")
                .from(Alias::new("participants"), Alias::new("entry_id"))
                .to(Alias::new("entries"), Alias::new("id"))
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ix_participants_workspace_id")
                .table(Alias::new("participants"))
                .col(Alias::new("workspace_id"))
                .to_owned(),
        )
            .await?;
        m.create_foreign_key(
            ForeignKey::create()
                .name("fk_participants_workspace")
                .from(Alias::new("participants"), Alias::new("workspace_id"))
                .to(Alias::new("workspaces"), Alias::new("id"))
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::SetNull) // good choice for optional
                .to_owned(),
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ux_participants_entry_workspace")
                .table(Alias::new("participants"))
                .col(Alias::new("entry_id"))
                .col(Alias::new("workspace_id"))
                .unique()
                .to_owned(),
        )
            .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Alias::new("participants")).to_owned())
            .await?;
        Ok(())
    }
}

