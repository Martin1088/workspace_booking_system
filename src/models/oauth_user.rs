use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};
use itertools::Itertools;
use rand::prelude::SliceRandom;
use loco_oauth2::models::users::OAuth2UserTrait;
use loco_rs::{auth::jwt, hash, prelude::*};
use super::o_auth2_sessions;
use async_trait::async_trait;
use chrono::offset::Local;
use crate::models::_entities::users;
use crate::models::users::Model;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OAuth2UserProfile {
    pub iss: Option<String>,
    pub sub: String,
    pub aud: Option<String>,
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub auth_time: Option<u64>,
    pub acr: Option<String>,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub given_name: Option<String>,
    pub preferred_username: Option<String>,
    pub nickname: Option<String>,
    pub groups: Option<Vec<String>>,
}
#[async_trait]
impl OAuth2UserTrait<OAuth2UserProfile> for Model {
    /// Asynchronously finds user by OAuth2 session id.
    /// # Arguments
    /// * `db` - Database connection
    /// * `cookie` - OAuth2 session id
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    /// * `ModelError` - When could not find the user in the DB
    async fn find_by_oauth2_session_id(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> ModelResult<Self> {
        // find the session by the session id
        let session = o_auth2_sessions::Entity::find()
            .filter(super::_entities::o_auth2_sessions::Column::SessionId.eq(session_id))
            .one(db)
            .await?
            .ok_or_else(|| ModelError::EntityNotFound)?;
        // if the session is found, find the user by the user id
        let user = users::Entity::find()
            .filter(users::Column::Id.eq(session.user_id))
            .one(db)
            .await?;
        user.ok_or_else(|| ModelError::EntityNotFound)
    }
    /// Asynchronously upsert user with OAuth data and saves it to the
    /// database.
    /// # Arguments
    /// * `db` - Database connection
    /// * `profile` - OAuth profile
    ///
    /// # Returns
    /// * `Self` - The `OAuth2UserTrait` struct
    ///
    /// # Errors
    ///
    /// When could not save the user into the DB
    async fn upsert_with_oauth(
        db: &DatabaseConnection,
        profile: &OAuth2UserProfile,
    ) -> ModelResult<Self> {
        let txn = db.begin().await?;
        let user = match users::Entity::find()
            .filter(users::Column::Email.eq(&profile.email))
            .one(&txn)
            .await?
        {
            None => {
                let password = generate_password(12);
                // We use the sub field as the user fake password since sub is unique
                let password_hash =
                    hash::hash_password(&password).map_err(|e| ModelError::Any(e.into()))?;
                // Admin check
                // Create the user into the database
                users::ActiveModel {
                    email: ActiveValue::set(profile.email.to_string()),
                    name: ActiveValue::set(profile.name.to_string()),
                    email_verified_at: ActiveValue::set(Some(Local::now().into())),
                    password: ActiveValue::set(password_hash),
                    pid: ActiveValue::set(Uuid::new_v4().as_bytes().to_vec()),
                    api_key: ActiveValue::set(Uuid::new_v4().to_string()),
                    is_admin: ActiveValue::set(
                        match &profile.groups {
                            Some(groups) => {
                                let allow = admin_groups_from_env();
                                let admin_flag = groups.iter().any(|g|
                                    allow.iter().any(|a| a.eq(g.trim()))
                                );
                                Some(if admin_flag { 1 } else { 0 })
                            },
                            None => None,
                        }
                    ),
                    ..Default::default()
                }
                    .insert(&txn)
                    .await
                    .map_err(|e| {
                        tracing::error!("Error while trying to create user: {e}");
                        ModelError::Any(e.into())
                    })?
            }
            // Do nothing if user exists
            Some(user) => user,
        };
        txn.commit().await?;
        Ok(user)
    }

    /// Generates a JWT
    /// # Arguments
    /// * `secret` - JWT secret
    /// * `expiration` - JWT expiration time
    ///
    /// # Returns
    /// * `String` - JWT token
    ///
    /// # Errors
    /// * `ModelError` - When could not generate the JWT
    fn generate_jwt(&self, secret: &str, expiration: &u64) -> ModelResult<String> {
        self.generate_jwt(secret, expiration)
    }
}

fn generate_password(length: usize) -> String {
    let lowercase = "abcdefghjkmnpqrstuvwxyz"; // excludes similar characters like 'i', 'l'
    let uppercase = "ABCDEFGHJKMNPQRSTUVWXYZ"; // excludes 'I', 'L', 'O'
    let numbers = "23456789";                 // excludes '0', '1'
    let symbols = "!@#$%^&*()-_=+[]{};:,.<>?";
    let spaces = " ";

    let mut charset = format!("{lowercase}{uppercase}{numbers}{symbols}{spaces}");
    let mut rng = thread_rng();

    // Generate password with characters from the full charset
    let mut password: Vec<char> = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset.chars().nth(idx).unwrap()
        })
        .collect();
    
    password.shuffle(&mut rng);
    password.iter().collect()
}

fn admin_groups_from_env() -> HashSet<String> {
    std::env::var("ADMIN_GROUPS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}