use std::time::Duration;

use crate::environment::read_numeric_env_var;
use axum::async_trait;
use axum_login::axum_sessions::async_session::{base64, Error, Session, SessionStore};
use chrono::Utc;

#[derive(Debug, Clone, Copy)]
pub struct CookieStore;

impl CookieStore {
    pub const fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SessionStore for CookieStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>, Error> {
        // should be an app-wide static; ideally only read once as it's not supposed to change
        let session_duration_minutes = read_numeric_env_var("SESSION_LIFETIME_MINUTES", &10u64);
        let serialized = base64::decode(cookie_value)?;
        let mut session: Session = bincode::deserialize(&serialized)?;
        session.set_expiry(Utc::now() + Duration::from_secs(session_duration_minutes * 60));

        Ok(session.validate())
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>, Error> {
        let serialized = bincode::serialize(&session)?;
        Ok(Some(base64::encode(serialized)))
    }

    async fn destroy_session(&self, _session: Session) -> Result<(), Error> {
        Ok(())
    }

    async fn clear_store(&self) -> Result<(), Error> {
        Ok(())
    }
}
