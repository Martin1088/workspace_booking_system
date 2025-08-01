use crate::models::help_fn::TimeFunctions;
use crate::models::mrbs_entry::{MrbsEntry, MrbsEntryActiveModel};
use crate::models::mrbs_participants::{MrbsParticipants, MrbsParticipantsActiveModel};
use crate::models::mrbs_room::{MrbsRoom, RoomData};
use crate::response_type::error::ErrorType;
use crate::response_type::success::ResponseType;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoomsQuery {
    pub room_id: i32,
    pub dates: Vec<String>,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoomIdQuery {
    pub room_id: i32,
    pub date: Option<String>,
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JoinRoomViaEntryIdQuery {
    pub entry_id: i32,
    pub username: String,
}

// timestamp as String 2023-02-02T00:00:00Z in rfc3339
pub async fn joinroom(
    db: &DatabaseConnection,
    entry_id: Option<i32>,
    room_id: Option<i32>,
    date: Option<String>,
    username: &str,
) -> Result<Response, Response> {
    let entry_id: i32 = match entry_id {
        Some(id_known) => id_known,
        None => match MrbsRoom::find_by_room_id(db, room_id.unwrap()).await? {
            Some(data) => {
                MrbsEntryActiveModel::set_entry_default(
                    db,
                    match date {
                        Some(d) => Some(
                            chrono::DateTime::parse_from_rfc3339(&d)
                                .map_err(|_| ErrorType::GivenTimeIsNotRCF3339.into_response())?,
                        ),
                        None => None,
                    },
                    room_id.unwrap(),
                    &data.room_name,
                    data.capacity,
                )
                .await?
            }
            None => Err(ErrorType::DBError.into_response())?,
        },
    };
    set_join_for_room(db, entry_id, username).await
}

// timestamp as String 2023-02-02T00:00:00Z in rfc3339
pub async fn joinrooms(
    db: &DatabaseConnection,
    room_id: i32,
    query_days: Vec<String>,
    username: &str,
) -> Result<Response, Response> {
    for date in query_days {
        let result: (i32, i32) = TimeFunctions::rfc3339_to_unixtimestamp(&date)
            .map_err(|_error| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
        let entry_id: Option<i32> =
            MrbsEntry::get_entry_id_by_day_range(db, room_id, result).await?;
        match entry_id {
            Some(id) => {
                set_join_for_room(db, id, username).await?;
            }
            None => {
                let data: RoomData = MrbsRoom::find_by_room_id(db, room_id)
                    .await
                    .map_err(|_error| ErrorType::DBError.into_response())?
                    .ok_or(ErrorType::DBError.into_response())?;
                let entry_id = MrbsEntryActiveModel::set_entry_default(
                    db,
                    Some(
                        chrono::DateTime::parse_from_rfc3339(&date)
                            .map_err(|_| ErrorType::GivenTimeIsNotRCF3339.into_response())?,
                    ),
                    room_id,
                    &data.room_name,
                    data.capacity,
                )
                .await?;
                set_join_for_room(db, entry_id, username).await?;
            }
        }
    }
    Ok(ResponseType::SuccessfulJoined.into_response())
}

async fn set_join_for_room(
    db: &DatabaseConnection,
    entry_id: i32,
    username: &str,
) -> Result<Response, Response> {
    // check with count() the amount of registrations
    let check = MrbsParticipants::count_participants_by_entry_id(entry_id, db).await?;
    // get the limit fo registrations allowed
    match MrbsEntry::get_limit_by_entry_id(db, entry_id).await? {
        Some(l) => {
            if check as i32 > l {
                return Err(ErrorType::RoomIsFull.into_response());
            }
        }
        None => return Err(ErrorType::NoRoomLimitProvided.into_response()),
    }
    // book room or give Error back as result
    MrbsParticipantsActiveModel::set_participant(db, entry_id, username).await?;
    Ok(ResponseType::SuccessfulJoined.into_response())
}

#[axum::debug_handler]
pub async fn joinroom_id(
    State(ctx): State<AppContext>,
    Json(params): Json<JoinRoomIdQuery>,
) -> std::result::Result<Response, Response> {
    Ok(joinroom(
        &ctx.db,
        None,
        Some(params.room_id.clone()),
        params.date.clone(),
        &params.username,
    )
    .await?)
}

#[axum::debug_handler]
pub async fn joinroom_via_entry_id(
    State(ctx): State<AppContext>,
    Json(params): Json<JoinRoomViaEntryIdQuery>,
) -> std::result::Result<Response, Response> {
    Ok(joinroom(
        &ctx.db,
        Some(params.entry_id.clone()),
        None,
        None,
        &params.username,
    )
    .await?)
}

#[axum::debug_handler]
pub async fn joinrooms_dates(
    State(ctx): State<AppContext>,
    Json(params): Json<JoinRoomsQuery>,
) -> std::result::Result<Response, Response> {
    Ok(joinrooms(
        &ctx.db,
        params.room_id.clone(),
        params.dates.clone(),
        &params.username,
    )
    .await?)
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/update/")
        .add("/joinroom_id", post(joinroom_id))
        .add("/joinroom_via_entry_id", post(joinroom_via_entry_id))
        .add("/joinrooms_dates", post(joinrooms_dates))
}
