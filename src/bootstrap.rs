use std::env;
use std::time::Duration;

use axum_login::axum_sessions::{PersistencePolicy, SameSite, SessionLayer};
use axum_login::axum_sessions::async_session::SessionStore;

pub fn read_secret_from_env() -> [u8; 64] {
    let secret = env::var("APP_SECRET").expect("App Secret is either undefined or not exactly 64 char long!");
    let mut secret_bytes = [0; 64];
    secret_bytes.copy_from_slice(secret.as_bytes());

    secret_bytes
}

pub fn build_session_layer(
    session_storage: impl SessionStore,
    secret: &[u8; 64],
) -> SessionLayer<impl SessionStore> {
    let session_duration_minutes = env::var("SESSION_LIFETIME_MINUTES")
        .unwrap_or("10".to_string())
        .parse::<u64>()
        .expect("Invalid session lifetime; can't convert to numeric value");
    let is_secure_cookie = env::var("SECURE_COOKIE")
        .unwrap_or("true".to_string())
        .to_ascii_lowercase()
        .eq("true");
    let session_layer = SessionLayer::new(session_storage, secret)
        .with_persistence_policy(PersistencePolicy::ExistingOnly)
        .with_session_ttl(Some(Duration::from_secs(session_duration_minutes * 60)))
        .with_same_site_policy(SameSite::Strict)
        .with_http_only(true)
        .with_secure(is_secure_cookie);

    session_layer
}
