#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;
mod m250714_000002_oauth2_sessions;

mod m20260105_122120_areas;
mod m20260105_125119_rooms;
mod m20260109_154036_entries;
mod m20260109_154705_participants;
mod m20260112_085153_workspaces;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20260105_122120_areas::Migration),
            Box::new(m20260105_125119_rooms::Migration),
            Box::new(m20260109_154036_entries::Migration),
            Box::new(m20260109_154705_participants::Migration),
            Box::new(m20260112_085153_workspaces::Migration),
            // inject-above (do not remove this comment)
            Box::new(m250714_000002_oauth2_sessions::Migration)
        ]
    }
}