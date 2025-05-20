use sea_orm::entity::prelude::*;
pub use super::_entities::mrbs_zoneinfo::{ActiveModel, Model, Entity};
pub type MrbsZoneinfo = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
