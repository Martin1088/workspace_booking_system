pub use super::_entities::mrbs_room::{ActiveModel, Entity, Model};
use crate::controllers::admin::CreateRoomQuery;
use crate::models::_entities::mrbs_room;
use crate::models::mrbs_participants::ParticipantsList;
use crate::response_type::error::ErrorType;
use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::{FromQueryResult, QuerySelect, Set};
use serde::{Deserialize, Serialize};
pub type MrbsRoom = Entity;
pub type MrbsRoomActiveModel = ActiveModel;

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
    #[sea_orm(skip)]
    pub participants: Vec<ParticipantsList>,
}

#[derive(Deserialize, Serialize, Debug, FromQueryResult)]
#[sea_orm(entity = "Mrbs_Room")]
pub struct RoomData {
    pub room_name: String,
    pub description: String,
    pub capacity: i32,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create_room(
        db: &DatabaseConnection,
        area_id: &i32,
        room: &CreateRoomQuery,
    ) -> Result<(), Response> {
        let new_room = mrbs_room::ActiveModel {
            area_id: Set(area_id.to_owned()),
            room_name: Set(room.room_name.to_string()),
            sort_key: Set(room.room_name.to_string()),
            description: Set(Some(room.description.to_string())),
            capacity: Set(room.capacity),
            ..Default::default()
        };
        mrbs_room::Entity::insert(new_room)
            .exec(db)
            .await
            .map_err(|_| ErrorType::DBError.into_response())?;
        Ok(())
    }

    pub async fn update_room(
        db: &DatabaseConnection,
        room: &CreateRoomQuery,
        model_room: &Model,
    ) -> Result<(), Response> {
        let mut change_room: MrbsRoomActiveModel = model_room.clone().into();
        change_room.room_name = Set(room.room_name.to_string());
        change_room.sort_key = Set(room.room_name.to_string());
        change_room.description = Set(Some(room.description.to_string()));
        change_room.capacity = Set(room.capacity);
        change_room.area_id = Set(room.area_id);
        change_room
            .update(db)
            .await
            .map_err(|_e| ErrorType::DBError.into_response())?;
        Ok(())
    }
    
    pub async fn delete_room(
        db: &DatabaseConnection,
        room_id: &i32,
    ) -> Result<(), Response> {
        mrbs_room::Entity::delete_by_id(room_id.to_owned())
            .exec(db)
            .await
            .map_err(|_error| ErrorType::FailedDelete.into_response())?;
        Ok(())
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_area_id(
        db: &DatabaseConnection,
        area_id: i32,
    ) -> Result<Vec<RoomsRelatedData>, Response> {
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

    pub async fn find_by_room_id(
        db: &DatabaseConnection,
        room_id: i32,
    ) -> Result<Option<RoomData>, Response> {
        Ok(MrbsRoom::find_by_id(room_id)
            .select_only()
            .columns([
                mrbs_room::Column::RoomName,
                mrbs_room::Column::Description,
                mrbs_room::Column::Capacity,
            ])
            .into_model::<RoomData>()
            .one(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?)
    }
    pub async fn find_by_room_id_all(
        db: &DatabaseConnection,
        room_id: &i32,
    ) -> Result<Model, Response> {
        Ok(mrbs_room::Entity::find_by_id(room_id.to_owned())
            .one(db)
            .await
            .map_err(|_e| ErrorType::NoRoomIDFound.into_response())?
            .ok_or(ErrorType::NoRoomIDFound.into_response())?)
    }
}
