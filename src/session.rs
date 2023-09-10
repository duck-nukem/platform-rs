use axum::async_trait;
use axum_login::axum_sessions::async_session::{base64, Error, Session, SessionStore};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Serialize, Deserialize, FromRow)]
struct StoredSession {
    id: String,
    expiry: Option<String>,
    data: String,
}

#[derive(Debug, Clone)]
pub(crate) struct DatabaseSessionStore {
    pool: Pool<Postgres>,
}

impl DatabaseSessionStore {
    pub(crate) fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionStore for DatabaseSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>, Error> {
        let session_id =
            Session::id_from_cookie_value(&cookie_value).expect("Invalid Session Cookie provided");
        let mut connection = self.pool.acquire().await.expect("Can't connect to the DB");
        let session_lookup =
            sqlx::query_as::<Postgres, StoredSession>("SELECT * FROM sessions WHERE id = $1")
                .bind(session_id)
                .fetch_one(connection.as_mut())
                .await;

        match session_lookup {
            Ok(session) => {
                let serialized = base64::decode(session.data)?;
                let session: Session = bincode::deserialize(&serialized)?;

                Ok(session.validate())
            }
            Err(_) => Ok(None),
        }
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>, Error> {
        let mut connection = self.pool.acquire().await.expect("Can't connect to the DB");
        let query = match session.expiry() {
            None => sqlx::query(
                "INSERT INTO sessions (id, data) \
                VALUES ($1, $2) \
                ON CONFLICT ON CONSTRAINT sessions_pkey \
                DO UPDATE SET data = $2 \
                WHERE sessions.id = $1;",
            ),
            Some(expiry_date) => sqlx::query(
                "INSERT INTO sessions (expiry, id, data) \
                VALUES ($1, $2, $3) \
                ON CONFLICT ON CONSTRAINT sessions_pkey \
                DO UPDATE SET data = $3, expiry = $1 \
                WHERE sessions.id = $2;",
            )
            .bind(expiry_date.to_string()),
        };

        let serialized_data =
            bincode::serialize(&session).expect("Failed to serialize session data");
        let base64_encoded_data = base64::encode(serialized_data);
        query
            .bind(session.clone().id())
            .bind(base64_encoded_data.as_str())
            .fetch_optional(&mut connection)
            .await
            .expect("Unable to save session to DB");

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await.expect("Can't connect to the DB");
        sqlx::query("DELETE FROM sessions WHERE id = $1")
            .bind(session.id().to_string())
            .fetch_optional(&mut connection)
            .await
            .expect("Couldn't delete session");

        Ok(())
    }

    async fn clear_store(&self) -> Result<(), Error> {
        let mut connection = self.pool.acquire().await.expect("Can't connect to the DB");
        sqlx::query("DELETE FROM sessions")
            .fetch_optional(&mut connection)
            .await
            .expect("Couldn't delete all sessions");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use axum_login::axum_sessions::async_session::{Session, SessionStore};
    use sqlx::{PgPool, Postgres, Row};

    use crate::session::{DatabaseSessionStore, StoredSession};

    #[sqlx::test]
    async fn test_store_session(pool: PgPool) {
        let session_store = DatabaseSessionStore::new(pool.clone());
        let session = Session::default();

        session_store
            .store_session(session.clone().validate().unwrap())
            .await
            .expect("Failed to store session");

        let mut connection = pool.acquire().await.unwrap();
        let stored_session =
            sqlx::query_as::<Postgres, StoredSession>("SELECT * FROM sessions WHERE id = $1;")
                .bind(session.id().to_string())
                .fetch_one(connection.as_mut())
                .await
                .expect("Unable to find session by id, test failed!");
        assert_eq!(session.id(), stored_session.id);
    }

    #[sqlx::test]
    async fn test_load_session(pool: PgPool) {
        let session_store = DatabaseSessionStore::new(pool);
        session_store
            .clear_store()
            .await
            .expect("Unable to empty store");
        let session = Session::default();
        let cloned_session = session.clone();
        let stored_cookie_value = session_store
            .store_session(session.validate().unwrap())
            .await
            .expect("Failed to store session");

        let stored_session = session_store
            .load_session(stored_cookie_value.clone().unwrap())
            .await
            .expect("Couldn't load session");

        assert_eq!(stored_session.unwrap().id(), cloned_session.id());
    }

    #[sqlx::test]
    async fn test_delete_session(pool: PgPool) {
        let mut connection = pool.acquire().await.unwrap();
        let session_store = DatabaseSessionStore::new(pool);
        let session = Session::default();

        session_store
            .destroy_session(session.clone())
            .await
            .expect("Couldn't delete session");

        let results_count = sqlx::query("SELECT COUNT(*) as count FROM sessions WHERE id = $1")
            .bind(session.id())
            .fetch_one(&mut connection)
            .await
            .expect("Couldn't fetch results count")
            .get::<i64, &str>("count");
        assert_eq!(results_count, 0);
    }

    #[sqlx::test]
    async fn test_clear_sessions(pool: PgPool) {
        let mut connection = pool.acquire().await.unwrap();
        let session_store = DatabaseSessionStore::new(pool);

        session_store
            .clear_store()
            .await
            .expect("Couldn't clear session store");

        let results_count = sqlx::query("SELECT COUNT(*) as count FROM sessions")
            .fetch_one(&mut connection)
            .await
            .expect("Couldn't fetch results count")
            .get::<i64, &str>("count");
        assert_eq!(results_count, 0);
    }
}
