use crate::controllers::admin::CreateRoomQuery;
use crate::models::_entities::prelude::MrbsRoom;
use crate::models::mrbs_area::{MrbsArea, MrbsAreaActiveModel};
use crate::models::mrbs_room::MrbsRoomActiveModel;
use crate::response_type::error::ErrorType;
use crate::response_type::success::ResponseType;
use loco_rs::prelude::*;
use sea_orm::DatabaseConnection;

pub trait RoomOperation {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        params: CreateRoomQuery,
        key_id: Option<i32>,
        username: &str,
    ) -> Result<Response, Response>;
}

pub async fn operate_room<T: RoomOperation>(
    db: &DatabaseConnection,
    params: CreateRoomQuery,
    key_id: Option<i32>,
    operation: &T,
) -> Result<Response, Response> {
    operation
        .apply(
            db,
            params,
            key_id,
            "",
        )
        .await
}

pub struct CreateRoom;
impl RoomOperation for CreateRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        params: CreateRoomQuery,
        _key_id: Option<i32>,
        _username: &str,
    ) -> Result<Response, Response> {
        MrbsRoomActiveModel::create_room(
            db,
            &params.area_id,
            &params).await?;
        Ok(ResponseType::SuccessfulJoined.into_response())
    }
}

pub struct UpdateRoom;
impl RoomOperation for UpdateRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        params: CreateRoomQuery,
        key_id: Option<i32>,
        _username: &str,
    ) -> Result<Response, Response> {
        let room_id = key_id.ok_or_else(|| ErrorType::NoEntryIDFound.into_response())?;
        let mut room_model = MrbsRoom::find_by_room_id_all(db, &room_id).await?;
        MrbsRoomActiveModel::update_room(db, &params, &mut room_model).await?;
        Ok(ResponseType::SuccessfulJoined.into_response())
    }
}

pub struct DeleteRoom;
impl RoomOperation for DeleteRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        params: CreateRoomQuery,
        key_id: Option<i32>,
        _username: &str,
    ) -> Result<Response, Response> {
        let room_id = key_id.ok_or_else(|| ErrorType::NoEntryIDFound.into_response())?;
        MrbsRoomActiveModel::delete_room(db, &room_id).await?;
        Ok(ResponseType::SuccessfulDelete.into_response())
    }
}

pub async fn operate_area<T: AreaOperation>(
    db: &DatabaseConnection,
    key_id: Option<i32>,
    area_name: Option<String>,
    operation: &T,
) -> Result<Response, Response> {
    operation.apply(db, area_name, key_id).await
}

pub trait AreaOperation {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<Response, Response>;
}

pub struct CreateArea;
impl AreaOperation for CreateArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        _key_id: Option<i32>,
    ) -> Result<Response, Response> {
        let area_name = match area {
            Some(a) => a,
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No name".to_owned(),
                }
                .into_response())?
            }
        };
        MrbsAreaActiveModel::create_area(db, area_name).await?;
        Ok(ResponseType::SuccessfulJoined.into_response())
    }
}

pub struct UpdateArea;
impl AreaOperation for UpdateArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<Response, Response> {
        let area_name = match area {
            Some(a) => a,
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No name".to_owned(),
                }
                .into_response())?
            }
        };
        let change_area = MrbsArea::find_area_by_id_all(
            db,
            key_id.ok_or(ErrorType::NoEntryIDFound.into_response())?,
        )
        .await?;
        MrbsAreaActiveModel::update_area(db, &change_area, area_name).await?;
        Ok(ResponseType::SuccessfulJoined.into_response())
    }
}

pub struct DeleteArea;
impl AreaOperation for DeleteArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        _area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<Response, Response> {
        MrbsAreaActiveModel::delete_area(db, match key_id {
            Some(i) => i,
            None => return Err(ErrorType::NoEntryIDFound.into_response())?,
        }).await?;
        Ok(ResponseType::SuccessfulJoined.into_response())
    }
}
