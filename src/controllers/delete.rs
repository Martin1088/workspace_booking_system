use loco_rs::prelude::*;
use axum::debug_handler;
use chrono::FixedOffset;
use crate::models::default_entries::EntryOperation;
use crate::models::mrbs_entry::{MrbsEntry, MrbsEntryActiveModel};
use crate::models::mrbs_participants::ActiveModel;

#[debug_handler]
pub async fn delete_join(
    State(ctx): State<AppContext>,
    Path(id): Path<i32>
) -> Result<Response, Response> {
    ActiveModel::delete_participant(&ctx.db, id).await
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/delete/")
        .add("joinroom/{id}", delete(delete_join))
}
