use async_trait::async_trait;
use axum::Router as AxumRouter;
use axum_extra::extract::cookie::SameSite;
use axum_session_sqlx::SessionMySqlPool;
use loco_rs::config::{Config, Initializers};
use loco_rs::db::connect;
use loco_rs::prelude::*;
use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
struct AxumSessionCookieCfg {
    protected_url: Option<String>,
    cookie_secure: Option<bool>,
    cookie_path: Option<String>,
    cookie_domain: Option<String>,
    cookie_same_site: Option<String>,
}

// Mapping der loco.yaml → initializers.axum_session.cookie_config
fn read_axum_session_cfg(app_cfg: &Option<Initializers>) -> AxumSessionCookieCfg {
    app_cfg.clone()
        .expect("No config for axum_session")
        .get("axum_session.cookie_config")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or(AxumSessionCookieCfg {
            protected_url: None,
            cookie_secure: None,
            cookie_path: None,
            cookie_domain: None,
            cookie_same_site: None,
        })
}

pub struct AxumSessionInitializer;

#[async_trait]
impl Initializer for AxumSessionInitializer {
    fn name(&self) -> String {
        "axum-session".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let conn = connect(&ctx.config.database).await?;
        let t= &ctx.config;
        let c = ctx.db.get_mysql_connection_pool();
        let config_env = read_axum_session_cfg(&ctx.config.initializers);

        // Create the session store configuration
        let session_config = axum_session::SessionConfig::default()
            .with_table_name("sessions_table")
            .with_key(axum_session::Key::generate())
            .with_database_key(axum_session::Key::generate())
            .with_cookie_path(config_env.cookie_path.unwrap_or_else(|| "/".into()))
            .with_cookie_domain(config_env.cookie_domain.unwrap_or_else(|| "localhost".into()))
            .with_secure(config_env.cookie_secure.unwrap_or(false))
            .with_cookie_same_site(SameSite::Lax);
        
        let st = SessionMySqlPool::from(c.clone());
        // Create the session store
        let session_store =
            axum_session::SessionStore::<SessionMySqlPool>::new(
                Some(st), 
                session_config
            )
                .await
                .unwrap();
        // Add the session store to the AxumRouter as an extension
        let router = router.layer(axum_session::SessionLayer::new(session_store));
        Ok(router)
    }
}