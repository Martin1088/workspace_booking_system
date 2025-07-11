use loco_rs::prelude::{IntoResponse, Response};
use sea_orm::entity::prelude::*;
use sea_orm::{FromQueryResult, QuerySelect, Set};
pub use super::_entities::mrbs_area::{ActiveModel, Model, Entity};
use serde::{Deserialize, Serialize};
use crate::models::_entities::{mrbs_area};
use crate::models::mrbs_room::{MrbsRoom, RoomsRelatedData};
use crate::response_type::error::ErrorType;
pub type MrbsArea = Entity;
pub type MrbsAreaActiveModel = ActiveModel;

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
    #[sea_orm(skip)]
    pub rooms: Vec<RoomsRelatedData>,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create_area(db: &DatabaseConnection,area_name: String) -> Result<() ,Response> {
        let new_area = mrbs_area::ActiveModel {
            area_name: Set(Some(area_name.clone())),
            sort_key: Set(area_name),
            ..Default::default()
        };
        mrbs_area::Entity::insert(new_area)
            .exec(db)
            .await
            .map_err(|_| ErrorType::DBError.into_response())?;
        Ok(())
    }
    
    pub async fn update_area(db: &DatabaseConnection, model_area: &Model, area_name: String)  -> Result<(),Response> {
        let mut change_area: ActiveModel = model_area.clone().into();
        change_area.area_name = Set(Some(area_name.clone()));
        change_area.sort_key = Set(area_name.clone());
        change_area
            .update(db)
            .await
            .map_err(|_e| ErrorType::DBError.into_response())?;
        Ok(())
    }
    
    pub async fn delete_area(db: &DatabaseConnection, area_id: i32)  -> Result<(),Response> {
        let result = mrbs_area::Entity::delete_by_id(area_id)
            .exec(db)
            .await
            .map_err(|_error| ErrorType::FailedDelete.into_response())?;
        if result.rows_affected == 0 {
            return Err(ErrorType::NoEntryIDFound.into_response());
        }

        Ok(())
    }
}

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
    
    pub async fn find_area_by_id_all(db: &DatabaseConnection, area_id: i32) -> Result<Model, Response> {
         Ok(mrbs_area::Entity::find_by_id(area_id)
            .one(db)
            .await
            .map_err(|_e| ErrorType::NoEntryIDFound.into_response())?
            .ok_or(ErrorType::NoEntryIDFound.into_response())?)
    }
}
