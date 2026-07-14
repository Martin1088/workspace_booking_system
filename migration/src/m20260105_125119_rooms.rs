use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "rooms",
            &[
                ("id", ColType::PkAuto),
                ("disabled", ColType::SmallUnsigned),
                ("area_id", ColType::Integer),
                ("room_name", ColType::String),
                ("description", ColType::TextNull),
                ("capacity", ColType::Integer),
                ("room_admin_email", ColType::TextNull),
            ],
            &[],
        )
        .await?;
        m.create_index(
            Index::create()
                .name("ix_rooms_area_id")
                .table(Alias::new("rooms"))
                .col(Alias::new("area_id"))
                .to_owned(),
        )
        .await?;
        m.create_index(
            Index::create()
                .name("ux_rooms_area_room_name")
                .table(Alias::new("rooms"))
                .col(Alias::new("area_id"))
                .col(Alias::new("room_name"))
                .unique()
                .to_owned(),
        )
        .await?;
        m.create_foreign_key(
            ForeignKey::create()
                .name("fk_rooms_area")
                .from(Alias::new("rooms"), Alias::new("area_id"))
                .to(Alias::new("areas"), Alias::new("id"))
                .on_update(ForeignKeyAction::Cascade)
                .on_delete(ForeignKeyAction::Restrict) // matches your current entity
                .to_owned(),
        )
            .await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "rooms").await
    }
}
