use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use share_types_booking::planner_types::{ErrorType, ResponseType};

use crate::entities::{mrbs_area, mrbs_room};

use super::{default_entries::DeleteOperation, RoomOperation};

pub async fn operate_room<T: RoomOperation>(
    db: &DatabaseConnection,
    key_id: Option<i32>,
    params: Option<Vec<String>>,
    operation: &T,
) -> Result<ResponseType, ErrorType> {
    operation
        .apply(
            db,
            match key_id {
                Some(i) => i,
                None => return Err(ErrorType::NoEntryIDFound)?,
            },
            params,
            "",
        )
        .await
}

pub struct CreateRoom;
impl RoomOperation for CreateRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        key_id: i32,
        params: Option<Vec<String>>,
        _username: &str,
    ) -> Result<ResponseType, ErrorType> {
        let room_param = match params {
            Some(p) => p.clone(),
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No room data".to_owned(),
                })?
            }
        };
        let room_name = room_param[0].clone();
        let description = room_param[1].clone();
        let capacity = match room_param[2].clone().parse::<i32>() {
            Ok(c) => c,
            Err(_) => {
                return Err(ErrorType::NoEntry {
                    message: "capacity could not be converted to i32".to_owned(),
                })?
            }
        };
        let new_room = mrbs_room::ActiveModel {
            area_id: Set(key_id),
            room_name: Set(room_name.clone()),
            sort_key: Set(room_name.clone()),
            description: Set(Some(description)),
            capacity: Set(capacity),
            ..Default::default()
        };
        mrbs_room::Entity::insert(new_room)
            .exec(db)
            .await
            .map_err(|_| ErrorType::DBError)?;
        Ok(ResponseType::SuccessfulJoined)
    }
}

pub struct UpdateRoom;
impl RoomOperation for UpdateRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        key_id: i32,
        params: Option<Vec<String>>,
        _username: &str,
    ) -> Result<ResponseType, ErrorType> {
        let room_param = match params {
            Some(p) => p.clone(),
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No room data".to_owned(),
                })?
            }
        };

        let room_model: mrbs_room::Model = mrbs_room::Entity::find_by_id(key_id)
            .one(db)
            .await
            .map_err(|_e| ErrorType::NoRoomIDFound)?
            .ok_or(ErrorType::NoRoomIDFound)?;
        let mut change_room: mrbs_room::ActiveModel = room_model.into();
        change_room.room_name = Set(room_param[0].clone());
        change_room.description = Set(Some(room_param[1].clone()));
        change_room.capacity = Set(match room_param[2].clone().parse::<i32>() {
            Ok(c) => c,
            Err(_) => {
                return Err(ErrorType::NoEntry {
                    message: "capacity could not be converted to i32".to_owned(),
                })?
            }
        });
        change_room.area_id = Set(match room_param[3].clone().parse::<i32>() {
            Ok(c) => c,
            Err(_) => {
                return Err(ErrorType::NoEntry {
                    message: "capacity could not be converted to i32".to_owned(),
                })?
            }
        });
        change_room
            .update(db)
            .await
            .map_err(|_e| ErrorType::DBError)?;
        Ok(ResponseType::SuccessfulJoined)
    }
}

pub struct DeleteRoom;
impl RoomOperation for DeleteRoom {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        key_id: i32,
        _params: Option<Vec<String>>,
        _username: &str,
    ) -> Result<ResponseType, ErrorType> {
        mrbs_room::Entity::delete_by_id(key_id)
            .exec(db)
            .await
            .map_err(|_error| ErrorType::FailedDelete)?;
        Ok(ResponseType::SuccessfulDelete)
    }
}

pub async fn operate_area<T: AreaOperation>(
    db: &DatabaseConnection,
    key_id: Option<i32>,
    area_name: Option<String>,
    operation: &T,
) -> Result<ResponseType, ErrorType> {
    operation.apply(db, area_name, key_id).await
}

pub trait AreaOperation {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<ResponseType, ErrorType>;
}

pub struct CreateArea;
impl AreaOperation for CreateArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        _key_id: Option<i32>,
    ) -> Result<ResponseType, ErrorType> {
        let area_name = match area {
            Some(a) => a,
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No name".to_owned(),
                })?
            }
        };
        let new_area = mrbs_area::ActiveModel {
            area_name: Set(Some(area_name.clone())),
            sort_key: Set(area_name),
            ..Default::default()
        };
        mrbs_area::Entity::insert(new_area)
            .exec(db)
            .await
            .map_err(|_| ErrorType::DBError)?;
        Ok(ResponseType::SuccessfulJoined)
    }
}

pub struct UpdateArea;
impl AreaOperation for UpdateArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<ResponseType, ErrorType> {
        let area_name = match area {
            Some(a) => a,
            None => {
                return Err(ErrorType::NoEntry {
                    message: "No name".to_owned(),
                })?
            }
        };
        let mut change_area: mrbs_area::ActiveModel = mrbs_area::Entity::find_by_id(match key_id {
            Some(i) => i,
            None => return Err(ErrorType::NoEntryIDFound)?,
        })
        .one(db)
        .await
        .map_err(|_e| ErrorType::NoEntryIDFound)?
        .ok_or(ErrorType::NoEntryIDFound)?
        .into();
        change_area.area_name = Set(Some(area_name.clone()));
        change_area.sort_key = Set(area_name.clone());
        change_area
            .update(db)
            .await
            .map_err(|_e| ErrorType::DBError)?;
        Ok(ResponseType::SuccessfulJoined)
    }
}

pub struct DeleteArea;
impl AreaOperation for DeleteArea {
    async fn apply(
        &self,
        db: &DatabaseConnection,
        _area: Option<String>,
        key_id: Option<i32>,
    ) -> Result<ResponseType, ErrorType> {
        mrbs_area::Entity::delete_by_id(match key_id {
            Some(i) => i,
            None => return Err(ErrorType::NoEntryIDFound)?,
        })
        .exec(db)
        .await
        .map_err(|_error| ErrorType::FailedDelete)?;
        Ok(ResponseType::SuccessfulJoined)
    }
}
