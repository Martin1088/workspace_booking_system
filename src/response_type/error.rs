use axum::http::StatusCode;
use loco_rs::prelude::{IntoResponse, Json, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

// error types can be set in the project
#[derive(Deserialize, Serialize, Debug)]
pub enum ErrorType {
    FailedDelete,
    DBError,
    RoomIsFull,
    NoRoomLimitProvided,
    NoRoomWithThisID,
    GivenTimeIsNotRCF3339,
    NoEntryForThisRoom,
    NoEntryIDFound,
    NoRoomIDFound,
    NoDateFound,
    NoEntry { message: String },
    FailedJoinOn { date: String },
    WeekOverviewCoundNotBeProvided,
}

impl IntoResponse for ErrorType {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            ErrorType::FailedDelete => (StatusCode::BAD_REQUEST, "Failed to delete".to_string()),
            ErrorType::DBError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ErrorType::RoomIsFull => (StatusCode::BAD_REQUEST, "Room is full".to_string()),
            ErrorType::NoRoomLimitProvided => (StatusCode::BAD_REQUEST, "No room limit provided".to_string()),
            ErrorType::NoRoomWithThisID => (StatusCode::NOT_FOUND, "No room with this ID".to_string()),
            ErrorType::GivenTimeIsNotRCF3339 => (StatusCode::BAD_REQUEST, "Time is not RFC3339".to_string()),
            ErrorType::NoEntryForThisRoom => (StatusCode::NOT_FOUND, "No entry for this room".to_string()),
            ErrorType::NoEntryIDFound => (StatusCode::NOT_FOUND, "No entry ID found".to_string()),
            ErrorType::NoRoomIDFound => (StatusCode::NOT_FOUND, "No room ID found".to_string()),
            ErrorType::NoDateFound => (StatusCode::BAD_REQUEST, "No date found".to_string()),
            ErrorType::NoEntry { message } => (StatusCode::NOT_FOUND, message.trim().to_string()),
            ErrorType::FailedJoinOn { date } => (
                StatusCode::BAD_REQUEST,
                format!("Failed to join on {}", date).trim().to_string(),
            ),
            ErrorType::WeekOverviewCoundNotBeProvided => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Week overview could not be provided".to_string(),
            ),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}