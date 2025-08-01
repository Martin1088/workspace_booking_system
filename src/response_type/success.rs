use axum::http::StatusCode;
use loco_rs::prelude::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use crate::models::mrbs_area::{AreaRelatedData, WeekOverview};
use crate::models::mrbs_room::RoomsRelatedData;
use serde_json::json;

// Type for successfull Response with data
#[derive(Deserialize, Serialize, Debug)]
pub enum ResponseType {
    SuccessfulJoined,
    SuccessfulDelete,
    ResultOverviewday { result: Vec<AreaRelatedData> },
    ResultRoom { result: RoomsRelatedData },
    ResultOverviewweek { result: Vec<WeekOverview> },
    CheckUserIfAdmin { isadmin: bool },
    RangeOfDefaultEntries { start: String, end: String },
    AllUsers{ result: Vec<String> }
}

impl IntoResponse for ResponseType {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ResponseType::SuccessfulJoined => (
                StatusCode::OK,
                json!({ "message": "Successfully joined" }),
            ),
            ResponseType::SuccessfulDelete => (
                StatusCode::OK,
                json!({ "message": "Successfully deleted" }),
            ),
            ResponseType::ResultOverviewday { result } => (
                StatusCode::OK,
                json!({ "result": result }),
            ),
            ResponseType::ResultRoom { result } => (
                StatusCode::OK,
                json!({ "result": result }),
            ),
            ResponseType::ResultOverviewweek { result } => (
                StatusCode::OK,
                json!({ "result": result }),
            ),
            ResponseType::CheckUserIfAdmin { isadmin } => (
                StatusCode::OK,
                json!({ "isadmin": isadmin }),
            ),
            ResponseType::RangeOfDefaultEntries { start, end } => (
                StatusCode::OK,
                json!({ "start": start, "end": end }),
            ),
            ResponseType::AllUsers { result } => (
                StatusCode::OK,
                json!({ "result": result }),
                ),
        };

        (status, Json(body)).into_response()
    }
}