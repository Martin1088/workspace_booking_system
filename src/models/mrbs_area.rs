use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::{FromQueryResult, QuerySelect};
pub use super::_entities::mrbs_area::{ActiveModel, Model, Entity};
pub type MrbsArea = Entity;
use serde::{Deserialize, Serialize};
use crate::models::_entities::{mrbs_area};
use crate::models::mrbs_room::{MrbsRoom, RoomsRelatedData};
use crate::response_type::error::ErrorType;

#[derive(Deserialize, Serialize, Debug)]
pub struct WeekOverview {
    pub weekday: String,
    pub date: String,
    pub areas: Vec<AreaRelatedData>,
}

#[derive(Deserialize, Serialize, Debug, FromQueryResult)]
#[sea_orm(entity = "Mrbs_Area")]
pub struct AreaRelatedData {
    pub id: i32,
    pub area_name: String,
    #[ sea_orm(skip)]
    pub rooms: Vec<RoomsRelatedData>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn collect_area_rooms(
        db: &DatabaseConnection,
    ) -> Result<Vec<AreaRelatedData>, Response> {
        let mut area: Vec<AreaRelatedData> = MrbsArea::find()
            .select_only()
            .columns([mrbs_area::Column::AreaName, mrbs_area::Column::Id])
            .into_model::<AreaRelatedData>()
            .all(db)
            .await
            .map_err(|_error| ErrorType::DBError.into_response())?;
        // for all areas the related rooms
        for a in &mut area {
            a.rooms = MrbsRoom::find_by_area_id(db, a.id).await?;
        }
        Ok(area)
    }
}
