use chrono::Utc;
use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::{QuerySelect, Set};
use serde::{Deserialize, Serialize};
use crate::models::_entities::mrbs_participants;
use crate::response_type::error::ErrorType;
pub use super::_entities::mrbs_participants::{ActiveModel, Model, Entity};
pub type MrbsParticipants = Entity;
pub type MrbsParticipantsActiveModel = ActiveModel;

// Mrbs_Participants
#[derive(Deserialize, Serialize, Debug, sea_orm::FromQueryResult)]
#[sea_orm(entity = "Mrbs_Participants")]
pub struct ParticipantsList {
    pub id: i32,
    pub username: String,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn set_participant(db: &DatabaseConnection, entry_id: i32, username: &str) -> Result<(), Response> {
        let new_join = mrbs_participants::ActiveModel {
            entry_id: Set(entry_id),
            username: Set(Some(username.to_owned())),
            create_by: Set(Some(username.to_owned())),
            registered: Set(Some(Utc::now().timestamp() as i32)),
            ..Default::default()
        };
        new_join
            .save(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?;
        Ok(())
    }
    pub async fn delete_participant(db: &DatabaseConnection, entry_id: i32) -> Result<(), Response> {
        MrbsParticipants::delete_many()
            .filter(mrbs_participants::Column::EntryId.eq(entry_id))
            .exec(db)
            .await
            .map_err(|_error| ErrorType::FailedDelete.into_response())?;
        Ok(())
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_entry_id(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<Vec<ParticipantsList>, Response> {
        Ok(MrbsParticipants::find()
            .filter(mrbs_participants::Column::EntryId.eq(id))
            .select_only()
            .columns([
                mrbs_participants::Column::Username,
                mrbs_participants::Column::Id,
            ])
            .into_model::<ParticipantsList>()
            .all(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?)
    }
    
    pub async fn count_participants_by_entry_id(entry_id: i32, db: &DatabaseConnection) -> Result<u64, Response> {
        Ok(MrbsParticipants::find()
            .having(mrbs_participants::Column::EntryId.eq(entry_id))
            .count(db)
            .await
            .map_err(|_error| ErrorType::NoEntry {
                message: _error.to_string(),
            }.into_response())?)
    }
}
