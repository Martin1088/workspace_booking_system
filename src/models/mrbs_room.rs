use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::{FromQueryResult, QuerySelect};
use serde::{Deserialize, Serialize};
use crate::models::_entities::mrbs_room;
use crate::models::mrbs_participants::ParticipantsList;
use crate::response_type::error::ErrorType;
pub use super::_entities::mrbs_room::{ActiveModel, Model, Entity};
pub type MrbsRoom = Entity;

#[derive(Deserialize, Serialize, Debug, FromQueryResult)]
#[sea_orm(entity = "Mrbs_Room")]
pub struct RoomsRelatedData {
    pub id: i32,
    pub room_name: String,
    pub description: String,
    pub capacity: i32,
    #[sea_orm(skip)]
    pub entry_id: Option<i32>,
    #[sea_orm(skip)]
    pub registration_limit: i32,
    #[ sea_orm(skip)]
    pub participants: Vec<ParticipantsList>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_area_id(db: &DatabaseConnection, area_id: i32) -> Result<Vec<RoomsRelatedData>, Response> {
        let room_collection = MrbsRoom::find()
            .filter(mrbs_room::Column::AreaId.eq(area_id))
            .select_only()
            .columns([
                mrbs_room::Column::Id,
                mrbs_room::Column::RoomName,
                mrbs_room::Column::Description,
                mrbs_room::Column::Capacity,
            ])
            .into_model::<RoomsRelatedData>()
            .all(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?;
        Ok(room_collection)
    }
}
