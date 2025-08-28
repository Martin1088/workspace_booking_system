use std::collections::HashSet;
use std::error::Error as StdError;
use std::fmt::Debug;
use axum::Extension;
use axum::extract::{Query, State};
use axum::routing::get;
use axum_session::{DatabasePool, Session, SessionLayer, SessionStore};
use axum_session_sqlx::SessionMySqlPool;
use loco_oauth2::controllers::middleware::{OAuth2CookieUser, OAuth2PrivateCookieJar};
use loco_oauth2::controllers::oauth2::{get_authorization_url, AuthParams};
use loco_oauth2::OAuth2ClientStore;
use loco_rs::Error;
use loco_oauth2::controllers::oauth2::callback;
use loco_oauth2::grants::authorization_code::GrantTrait;
use loco_rs::app::AppContext;
use loco_rs::controller::{format, unauthorized, Routes};
use loco_rs::prelude::{IntoResponse, Response};
use crate::models::{o_auth2_sessions, users};
use crate::models::oauth_user::OAuth2UserProfile;
use crate::views::auth::LoginResponse;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::url::Url;
use serde::Deserialize;
use tokio::sync::MutexGuard;
use tracing::info;

pub async fn get_authorization_url_with_pkce<T>(
    mut session: Session<T>,
    oauth2_client: &mut MutexGuard<'_, dyn GrantTrait>,
) -> String
where
    T: axum_session::DatabasePool + Clone + Debug + Sync + Send + 'static,
{
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    session
        .set("PKCE_VERIFIER", pkce_verifier.secret().to_owned());
    let (auth_url, csrf) = oauth2_client.get_authorization_url();
    session
        .set("CSRF_TOKEN", csrf.secret().to_owned());
    let mut url = Url::parse(&auth_url.to_string()).expect("invalid auth_url");
    {
        let mut qp = url.query_pairs_mut();
        qp.append_pair("code_challenge", pkce_challenge.as_str());
        qp.append_pair("code_challenge_method", "S256");
    }
    url.to_string()
}
/*
pub async fn authentik_authorization_url(
    mut session: Session<SessionMySqlPool>,
) -> Result<String, loco_rs::Error> {
    
    let client = BasicClient::new(
        ClientId::new(std::env::var("OAUTH_CLIENT_ID")?),
        Some(ClientSecret::new(std::env::var("OAUTH_CLIENT_SECRET")?)),
        AuthUrl::new(std::env::var("AUTH_URL")?)?,
        Some(TokenUrl::new(std::env::var("TOKEN_URL")?)?)
    ).set_redirect_uri(RedirectUrl::new(std::env::var("REDIRECT_URL")?)?);

    // PKCE
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    session.set("PKCE_VERIFIER", verifier.secret().to_string());

    // CSRF
    let (auth_url, state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("openid".into()))
        .add_scope(oauth2::Scope::new("email".into()))
        .add_scope(oauth2::Scope::new("profile".into()))
        .set_pkce_challenge(challenge)
        .url();

    session.set("CSRF_TOKEN", state.secret().to_string());
    Ok(auth_url.to_string())
}
*/
pub async fn authentik_authorization_url(
    session: Session<SessionMySqlPool>,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<String, Error> {
    let mut client = oauth2_store
        .get_authorization_code_client("authentik") // changed here
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    
    let auth_url = get_authorization_url(session, &mut client).await;
    info!("Auth URL: {}", auth_url);
    drop(client);
    Ok(auth_url)
}



#[derive(Debug, Deserialize)]
pub struct AuthP {
    pub code: String,
    pub state: Option<String>,
}
/// The callback URL for the `OAuth2` flow
/// This will exchange the code for a token and then get the user profile
/// then upsert the user and the session and set the token in a short live
/// cookie Lastly, it will redirect the user to the protected URL
/// # Arguments
/// * `ctx` - The application context
/// * `session` - The axum session
/// * `params` - The query parameters
/// * `jar` - The oauth2 private cookie jar
/// * `oauth_store` - The `OAuth2ClientStore` extension
/// # Returns
/// The response with the short live cookie and the redirect to the protected
/// URL
/// # Errors
/// * `loco_rs::errors::Error`
pub async fn authentik_callback_cookie(
    State(ctx): State<AppContext>,
    session: Session<SessionMySqlPool>,
    Query(params): Query<AuthParams>,
    jar: OAuth2PrivateCookieJar,
    Extension(oauth2_store): Extension<OAuth2ClientStore>,
) -> Result<impl IntoResponse, Error> {
    info!("cookie: {:?}", session);
    
    let mut client = oauth2_store
        .get_authorization_code_client("authentik")
        .await
        .map_err(|e| {
            tracing::error!("Error getting client: {:?}", e);
            Error::InternalServerError
        })?;
    
    // This function will validate the state from the callback. Then it will exchange the code for a token and then get the user profile. After that, the function will upsert the user and the session and set the token in a short live cookie and save the cookie in the private cookie jar. Lastly, the function will create a response with the short live cookie and the redirect to the protected URL
    info!("Exchanging code for token...");
    info!(" client cookie{:?}",client.get_cookie_config());
    info!("parms: {:?}", params);
    info!("session: {:?}",session);
    let response = callback::<OAuth2UserProfile, users::Model, o_auth2_sessions::Model, SessionMySqlPool>(
        ctx,
        session,
        params,
        jar,
        &mut client,
    )
        .await
        .map_err(|e| {
            tracing::error!("OAuth2 callback failed: {:?}", e);
            if let Some(source) = e.source() {
                tracing::error!("Caused by: {:?}", source);
            }
            e
        })?;
    drop(client);
    Ok(response)
}



async fn protected(
    State(ctx): State<AppContext>,
    // Extract the user from the Cookie via middleware
    user: OAuth2CookieUser<OAuth2UserProfile, users::Model, o_auth2_sessions::Model>,
) -> Result<Response, Error> {
    info!("user: {:?}", user);
    let user: &users::Model = user.as_ref();
    // groups check for admin
    
    let jwt_secret = ctx.config.get_jwt_config()?;
    // Generate a JWT token
    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;
    
    // Return the user and the token in JSON format
    format::json(LoginResponse::new(user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/oauth2")
        .add("/authentik", get(authentik_authorization_url))
        // Route for the Cookie callback
        .add(
            "/authentik/callback/cookie",
            get(authentik_callback_cookie),
        )
        .add("/protected", get(protected))
}