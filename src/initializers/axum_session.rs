use async_trait::async_trait;
use axum::Router as AxumRouter;
use axum_session_sqlx::SessionMySqlPool;
use loco_rs::db::connect;
use loco_rs::prelude::*;

pub struct AxumSessionInitializer;

#[async_trait]
impl Initializer for AxumSessionInitializer {
    fn name(&self) -> String {
        "axum-session".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let conn = connect(&ctx.config.database).await?;
        let c = ctx.db.get_mysql_connection_pool();
        // Create the session store configuration
        let session_config = axum_session::SessionConfig::default()
            .with_table_name("sessions_table")
            .with_key(axum_session::Key::generate())
            .with_database_key(axum_session::Key::generate());
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