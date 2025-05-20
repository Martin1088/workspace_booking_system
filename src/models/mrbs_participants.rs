use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::QuerySelect;
use serde::{Deserialize, Serialize};
use crate::models::_entities::mrbs_participants;
use crate::response_type::error::ErrorType;
pub use super::_entities::mrbs_participants::{ActiveModel, Model, Entity};
pub type MrbsParticipants = Entity;

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
impl ActiveModel {}

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
}
