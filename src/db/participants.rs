use anyhow::{Context, Result};
use chrono::NaiveDate;
use uuid::Uuid;

use super::Gender;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Participant {
    pub id: Uuid,
    pub short_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub birthday: NaiveDate,
}

#[derive(Clone)]
pub struct Repository {
    pool: super::DatabasePool,
}

impl Repository {
    pub fn new(pool: super::DatabasePool) -> Self {
        Self { pool }
    }

    /// List all participants in the database.
    pub async fn list_participants(&self) -> Result<Vec<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
                SELECT
                    id, short_id, first_name, last_name, gender AS "gender: _",
                    birthday
                FROM participants;
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch list of all participants from database")
    }

    /// Get a participant by its id.
    ///
    /// # Returns
    /// - `Ok(Some(...))` - if the participant has been found
    /// - `Ok(None)` - if no participant with the given `participant_id` exists
    /// - `Error(...)` - in case of an database error
    pub async fn participant_by_id(&self, participant_id: Uuid) -> Result<Option<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
                SELECT
                    id, short_id, first_name, last_name, gender AS "gender: _",
                    birthday
                FROM participants
                WHERE id = $1;
            "#,
            participant_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch list of all participants from database")
    }

    /// Create a new participant in the database.
    pub async fn create_participant(
        &self,
        first_name: &str,
        last_name: &str,
        gender: Gender,
        birthday: NaiveDate,
    ) -> Result<Uuid> {
        sqlx::query_scalar!(
            r#"
                INSERT INTO participants (
                    first_name, last_name, gender, birthday
                ) VALUES (
                    $1, $2, $3, $4
                ) RETURNING id;
            "#,
            first_name,
            last_name,
            gender as Gender,
            birthday
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create participant in database")
    }
}
