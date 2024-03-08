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
    pub group_id: Uuid,
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
                    birthday, group_id
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
                    birthday, group_id
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
        group_id: Uuid,
    ) -> Result<Uuid> {
        sqlx::query_scalar!(
            r#"
                INSERT INTO participants (
                    first_name, last_name, gender, birthday, group_id
                ) VALUES (
                    $1, $2, $3, $4, $5
                ) RETURNING id;
            "#,
            first_name,
            last_name,
            gender as Gender,
            birthday,
            group_id,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to create participant in database")
    }

    /// Delete a participant from the database.
    ///
    /// # Returns
    /// - `Ok(true)` - if the participant has been deleted
    /// - `Ok(false)` - if the participant did not exist
    /// - `Err(e)` - in case of an database error
    pub async fn delete_participant(&self, participant_id: Uuid) -> Result<bool> {
        let rows = sqlx::query!(
            r#"
                DELETE FROM participants
                WHERE id = $1
            "#,
            participant_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete participant from database")?
        .rows_affected();

        Ok(rows > 0)
    }

    /// List participants in the group in the database.
    pub async fn list_participants_in_group(&self, group_id: Uuid) -> Result<Vec<Participant>> {
        sqlx::query_as!(
            Participant,
            r#"
                SELECT
                    id, short_id, first_name, last_name, gender AS "gender: _",
                    birthday, group_id
                FROM participants
                WHERE group_id = $1;
            "#,
            group_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch list of participants in group from database")
    }
}
