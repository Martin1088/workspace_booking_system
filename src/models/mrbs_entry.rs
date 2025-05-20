use loco_rs::prelude::*;
use sea_orm::{Condition, QuerySelect};
use sea_orm::entity::prelude::*;
use crate::models::_entities::mrbs_entry;
use crate::models::mrbs_participants::MrbsParticipants;
use crate::models::mrbs_room::RoomsRelatedData;
use crate::response_type::error::ErrorType;
pub use super::_entities::mrbs_entry::{ActiveModel, Model, Entity};
pub type MrbsEntry = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn find_by_day_range(
        db: &DatabaseConnection,
        rooms: &mut RoomsRelatedData,
        day_range: (i32, i32)
    ) -> Result<(), Response> {
        let condition_day = Condition::all()
            .add(mrbs_entry::Column::RoomId.eq(rooms.id))
            .add(mrbs_entry::Column::StartTime.gte(day_range.0))
            .add(mrbs_entry::Column::EndTime.lte(day_range.1));
        let entryid_limit: Option<(i32, i32)> = MrbsEntry::find()
            .filter(condition_day)
            .select_only()
            .columns(
                [mrbs_entry::Column::Id, mrbs_entry::Column::RegistrantLimit]
            )
            .into_tuple()
            .one(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?;
        // all participants for all rooms and there paricipants_id default 0
        match entryid_limit {
            Some(e) => {
                let participants_collection = MrbsParticipants::find_by_entry_id(  db, e.0).await?;
                rooms.participants = participants_collection;
                rooms.entry_id = Some(e.0);
                rooms.registration_limit = e.1;
            }
            None => {
                rooms.entry_id = None;
                rooms.registration_limit = rooms.capacity;
            }
        }
        Ok(())
    }
}
