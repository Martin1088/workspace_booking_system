use sea_orm::entity::prelude::*;
pub use super::_entities::o_auth2_sessions::{ActiveModel, Model, Entity};
use super::users;
use async_trait::async_trait;
use chrono::{Local, Utc};
use loco_oauth2::{
    base_oauth2::basic::BasicTokenResponse, base_oauth2::TokenResponse,
    models::oauth2_sessions::OAuth2SessionsTrait,
};
use loco_rs::prelude::*;
use sea_orm::entity::prelude::*;
use tracing::{debug, error};
use crate::models::_entities::o_auth2_sessions;

#[async_trait]
impl OAuth2SessionsTrait<users::Model> for Model {
    /// Check if a session is expired from the database
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// session_id: &str - Session id
    /// # Returns
    /// A boolean
    /// # Errors
    /// Returns a `ModelError` if the session is not found
    async fn is_expired(db: &DatabaseConnection, session_id: &str) -> ModelResult<bool> {
        let oauth2_session = o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        Ok(oauth2_session.expires_at < Utc::now())
    }

    /// Upsert a session with OAuth
    ///
    /// # Arguments
    /// db: &`DatabaseConnection` - Database connection
    /// token: &`BasicTokenResponse` - OAuth token
    /// user: &`users::Model` - User
    /// # Returns
    /// A session
    /// # Errors
    /// Returns a `ModelError` if the session cannot be upserted
    async fn upsert_with_oauth2(
        db: &DatabaseConnection,
        token: &BasicTokenResponse,
        user: &users::Model,
    ) -> ModelResult<Self> {
        debug!("Upsert");
        debug!("{:?}", user);
        let txn = db.begin().await?;
        let oauth2_session_id = token.access_token().secret().clone();
        let oauth2_session = match o_auth2_sessions::Entity::find()
            .filter(o_auth2_sessions::Column::UserId.eq(user.id))
            .one(&txn)
            .await?
        {
            Some(oauth2_session) => {
                // Update the session
                let mut oauth2_session: o_auth2_sessions::ActiveModel = oauth2_session.into();
                oauth2_session.session_id = ActiveValue::set(oauth2_session_id);
                oauth2_session.expires_at =
                    ActiveValue::set(Utc::now() + token.expires_in().unwrap());
                oauth2_session.updated_at = ActiveValue::set(Utc::now());
                oauth2_session.update(&txn).await?
            }
            None => {
                debug!("No session found for user, creating new session...");
                let new_session = o_auth2_sessions::ActiveModel {
                    session_id: ActiveValue::set(oauth2_session_id.clone()),
                    expires_at: ActiveValue::set(Utc::now() + token.expires_in().unwrap()),
                    user_id: ActiveValue::set(user.id),
                    updated_at: ActiveValue::set(Utc::now()),
                    created_at: ActiveValue::not_set(),
                    ..Default::default()
                };
                debug!("New session model to insert: {:?}", new_session);
                let inserted = new_session.insert(&txn).await;
                match &inserted {
                    Ok(val) => debug!("Session inserted: {:?}", val),
                    Err(err) => error!("Failed to insert session: {:?}", err),
                }
                inserted?
            }
        };
        debug!("Create {:?}", oauth2_session);
        txn.commit().await?;
        Ok(oauth2_session)
    }
}
pub type OAuth2Sessions = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
