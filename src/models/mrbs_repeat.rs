use axum::response::Response;
use chrono::FixedOffset;
use loco_rs::prelude::IntoResponse;
use sea_orm::entity::prelude::*;
use sea_orm::{QueryOrder, QuerySelect, Set};
use crate::models::_entities::mrbs_repeat;
use crate::models::_entities::mrbs_repeat::Column;
use crate::models::help_fn::TimeFunctions;
use crate::response_type::error::ErrorType;
pub use super::_entities::mrbs_repeat::{ActiveModel, Model, Entity};
pub type MrbsRepeat = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {

    pub async fn check_repeat_room(
        db: &DatabaseConnection,
        room_id: i32,
        timegiven: chrono::DateTime<FixedOffset>,
    ) -> loco_rs::Result<i32, Response> {
        let last_repeat: Option<(i32, i32)> = MrbsRepeat::find()
            .filter(mrbs_repeat::Column::RoomId.eq(room_id))
            .select_only()
            .columns([mrbs_repeat::Column::Id, mrbs_repeat::Column::EndDate])
            .order_by_desc(Column::EndDate)
            .limit(1)
            .into_tuple()
            .one(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?;
        match last_repeat {
            Some(repeat_entry) => {
                if timegiven
                    > chrono::DateTime::from_timestamp(repeat_entry.1.into(), 0)
                    .ok_or(ErrorType::GivenTimeIsNotRCF3339.into_response())?
                {
                    Self::set_repeat_enddate(db, repeat_entry.0, timegiven).await?;
                    Ok(repeat_entry.0)
                } else {
                    Ok(repeat_entry.0)
                }
            }
            None => todo!(),
        }
    }
    
    async fn set_repeat_enddate(
        db: &DatabaseConnection,
        repeat_id: i32,
        old_date: chrono::DateTime<FixedOffset>,
    ) -> loco_rs::Result<(), Response> {
        let mut change_repeat: mrbs_repeat::ActiveModel = MrbsRepeat::find_by_id(repeat_id)
            .one(db)
            .await
            .map_err(|_e| ErrorType::NoDateFound.into_response())?
            .ok_or(ErrorType::NoDateFound.into_response())?
            .into();
        change_repeat.end_date =
            Set(TimeFunctions::end_of_year(old_date).ok_or(ErrorType::NoDateFound.into_response())?);
        change_repeat
            .update(db)
            .await
            .map_err(|_e| ErrorType::DBError.into_response())?;
        Ok(())
    }

}
