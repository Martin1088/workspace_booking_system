use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::create_admin::{operate_area, operate_room, CreateArea, CreateRoom, DeleteArea, DeleteRoom, UpdateArea, UpdateRoom};
use crate::models::mrbs_room::MrbsRoomActiveModel;
use crate::response_type::success::ResponseType;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CreateRoomQuery {
    pub room_name: String,
    pub description: String,
    pub capacity: i32,
    pub area_id: i32,
    pub room_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAreaQuery {
    pub area_name: String,
}

pub async fn create_room(
    ctx: State<AppContext>,
    Json(params): Json<CreateRoomQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = MrbsRoomActiveModel::create_room(
        db,
        &params.area_id,
        &params).await?;
    Ok(ResponseType::SuccessfulJoined.into_response())
}

pub async fn delete_room(
    ctx: State<AppContext>,
    Path(room_id): Path<i32>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = operate_room(db, CreateRoomQuery::default(), Some(room_id), &DeleteRoom).await?;
    Ok(res.into_response())
}

pub async fn update_room(
    ctx: State<AppContext>,
    Path(room_id): Path<i32>,
    Json(params): Json<CreateRoomQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = operate_room(db, params, Some(room_id), &UpdateRoom).await?;
    Ok(res.into_response())
}

pub async fn create_area(
    ctx: State<AppContext>,
    Json(params): Json<CreateAreaQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = operate_area(db, None, Some(params.area_name),  &CreateArea).await?;
    Ok(res.into_response())
}
pub async fn delete_area(
    ctx: State<AppContext>,
    Path(area_id): Path<i32>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = operate_area(db, Option::from(area_id), None, &DeleteArea).await?;
    Ok(res.into_response())
}
pub async fn update_area(
    ctx: State<AppContext>,
    Path(area_id): Path<i32>,
    Json(params): Json<CreateAreaQuery>,
) -> Result<Response, Response> {
    let db: &DatabaseConnection = &ctx.db;
    let res = operate_area(db, Some(area_id), Some(params.area_name), &UpdateArea).await?;
    Ok(res.into_response())
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/admin")
        .add("/createroom", post(create_room))
        .add("/updateroom/{id}", post(update_room))
        .add("/deleteroom/{id}", delete(delete_room))
        .add("/createarea", post(create_area))
        .add("/deletearea/{id}", delete(delete_area))
}
