use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "areas",
            &[
                ("id", ColType::PkAuto),
                ("disabled", ColType::SmallUnsigned),
                ("area_name", ColType::String),
                ("area_admin_email", ColType::StringNull),            
            ],
            &[]
        ).await?;

        m.create_index(
            Index::create()
                .name("ux_areas_area_name")
                .table(Alias::new("areas"))
                .col(Alias::new("area_name"))
                .unique()
                .to_owned(),
        )
            .await?;
        Ok(())
    }
    

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "areas").await
    }
}
