use loco_rs::schema::{create_table, ColType};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "workspaces",
            &[
                ("id", ColType::PkAuto),
                ("room_id", ColType::Integer),
                ("name", ColType::String),
                ("workspace_type", ColType::String),
                ("description", ColType::TextNull),
                ("disabled", ColType::SmallUnsigned),
            ],
            &[],
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ix_workspaces_room_id")
                .table(Alias::new("workspaces"))
                .col(Alias::new("room_id"))
                .to_owned(),
        )
            .await?;
        m.create_index(
            Index::create()
                .name("ux_workspaces_room_name")
                .table(Alias::new("workspaces"))
                .col(Alias::new("room_id"))
                .col(Alias::new("name"))
                .unique()
                .to_owned(),
        )
            .await?;
        m.create_foreign_key(
            ForeignKey::create()
                .name("fk_workspaces_room")
                .from(Alias::new("workspaces"), Alias::new("room_id"))
                .to(Alias::new("rooms"), Alias::new("id"))
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Restrict)
                .to_owned(),
        )
            .await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        m.drop_table(Table::drop().table(Alias::new("workspaces")).to_owned())
            .await?;
        Ok(())
    }
}

