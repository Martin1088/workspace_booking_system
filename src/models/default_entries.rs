use chrono::{DateTime, Datelike, Duration, FixedOffset, Weekday};
use loco_rs::prelude::*;
use sea_orm::DatabaseConnection;
use crate::models::mrbs_area::{AreaRelatedData, MrbsArea};
use crate::models::mrbs_entry::{MrbsEntry, MrbsEntryActiveModel};
use crate::models::mrbs_participants::MrbsParticipantsActiveModel;
use crate::response_type::error::ErrorType;
use crate::response_type::success::ResponseType;

pub trait EntryOperation {
    fn apply(
        &self,
        db: &DatabaseConnection,
        current: DateTime<FixedOffset>,
        room_id: i32,
        room_name: &str,
        room_capacity: i32,
    ) -> impl std::future::Future<Output = Result<(), Response>> + Send;
}

pub struct CreateOperation;
impl EntryOperation for CreateOperation {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        current: DateTime<FixedOffset>,
        room_id: i32,
        room_name: &str,
        room_capacity: i32,
    ) -> Result<(), Response> {
        if MrbsEntry::check_entry_roomid(db, room_id, &current.to_rfc3339())
            .await?
            .is_none()
        {
            MrbsEntryActiveModel::set_entry_default(db, Some(current), room_id, room_name, room_capacity).await?;
        }
        Ok(())
    }
}


pub async fn operate_entry<T: EntryOperation>(
    db: &DatabaseConnection,
    param: Option<Vec<String>>,
    operation: &T,
) -> Result<Response, Response> {
    let start: DateTime<FixedOffset>;
    let end: DateTime<FixedOffset>;
    match param {
        Some(res) => {
            start = DateTime::parse_from_rfc3339(&res[0])
                .map_err(|_e| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
            end = DateTime::parse_from_rfc3339(&res[1])
                .map_err(|_e| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
        }
        None => return Err(ErrorType::NoDateFound.into_response())?,
    };
    let area_data: Vec<AreaRelatedData> = MrbsArea::collect_area_rooms(db).await?;
    let mut current = start;
    while current <= end {
        // No Weekend check
        if current.weekday() != Weekday::Sat && current.weekday() != Weekday::Sun {
            for a in &area_data {
                for room in &a.rooms {
                    operation
                        .apply(db, current, room.id, &room.room_name, room.capacity)
                        .await?;
                }
            }
        }
        current = current + Duration::days(1);
    }
    Ok(ResponseType::SuccessfulJoined.into_response())
}


pub async fn delete_entry(
    db: &DatabaseConnection,
    entry_id: i32,
) -> Result<(), Response> {
    MrbsEntryActiveModel::delete_entry_by_id(db, entry_id).await?;
    MrbsParticipantsActiveModel::delete_participant(db, entry_id).await?;
    Ok(())
}

