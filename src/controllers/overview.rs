use crate::controllers::help_fn::TimeFunctions;
use crate::models::mrbs_area::{AreaRelatedData, MrbsArea, WeekOverview};
use crate::models::mrbs_entry::MrbsEntry;
use crate::response_type::error::ErrorType;
use crate::response_type::success::ResponseType;
use axum::routing::get;
use chrono::{Datelike, Duration, Utc};
use loco_rs::app::AppContext;
use loco_rs::controller::Routes;
use loco_rs::prelude::{post, IntoResponse, Json, Response, State};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OverviewQuery {
    pub date: Option<String>,
}

pub async fn get_overview_day(
    db: &DatabaseConnection,
    date: String,
) -> Result<ResponseType, Response> {
    // get start and end of day
    let day_range: (i32, i32) = TimeFunctions::rfc3339_to_unixtimestamp(&date)
        .map_err(|_error| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
    // get all arears
    let mut area_room_collection: Vec<AreaRelatedData> = MrbsArea::collect_area_rooms(db).await?;
    // for all areas the related rooms
    // for all rooms the entry_id and registration_limit
    for areas in &mut area_room_collection {
        for rooms in &mut areas.rooms {
            MrbsEntry::find_by_day_range(db, rooms, day_range).await?;
        }
    }
    //result Datastruct with all areas, rooms and participants
    let res = ResponseType::ResultOverviewday {
        result: area_room_collection,
    };
    Ok(res)
}

pub async fn read_overview_day(
    State(ctx): State<AppContext>,
    Json(params): Json<OverviewQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    // param date as rfc3339 if None today
    let timegiven: String = params.date.unwrap_or_else(|| {
        let start_of_day = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap(); // only default time
        start_of_day.and_utc().to_rfc3339()
    });
    let res = get_overview_day(db, timegiven).await?;
    Ok(res.into_response())
}

pub async fn get_overview_week(
    ctx: State<AppContext>,
    Json(params): Json<OverviewQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let mut result_week: Vec<WeekOverview> = Vec::new();
    // param date as rfc3339 if None today
    let timegiven: String = params.date.unwrap_or_else(|| {
        let start_of_day = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap(); // only default time
        start_of_day.and_utc().to_rfc3339()
    });
    let mut date_week =
        TimeFunctions::get_monday(&timegiven).map_err(|_error| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
    let mut weekday_result = date_week.weekday();
    for _i in 0..5 {
        let dayview = get_overview_day(db, date_week.to_rfc3339()).await?;
        match dayview {
            ResponseType::ResultOverviewday { result } => {
                let result_day = WeekOverview {
                    weekday: weekday_result.to_string(),
                    date: date_week.to_rfc3339(),
                    areas: result,
                };
                result_week.push(result_day)
            }
            _ => return Err(ErrorType::WeekOverviewCoundNotBeProvided.into_response()),
        }
        date_week = date_week + Duration::days(1);
        weekday_result = weekday_result.succ();
    }
    Ok(ResponseType::ResultOverviewweek {
        result: result_week,
    }
    .into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("/api")
        .add("/overviewday", post(read_overview_day))
        .add("/overviewweek", post(get_overview_week))
}
