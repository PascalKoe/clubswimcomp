use anyhow::{Context as _, Result};
use uuid::Uuid;

pub struct Registration {
    pub id: Uuid,
    pub participant_id: Uuid,
    pub competition_id: Uuid,
}

pub struct RegistrationResult {
    pub registration_id: Uuid,
    pub disqualified: bool,
    pub time_millis: i64,
}

#[derive(Clone)]
pub struct Repository {
    pool: super::DatabasePool,
}

impl Repository {
    pub fn new(pool: super::DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn list_for_participant(&self, participant_id: Uuid) -> Result<Vec<Registration>> {
        sqlx::query_as!(
            Registration,
            r#"
                SELECT
                    id, participant_id, competition_id
                FROM registrations
                WHERE participant_id = $1;
            "#,
            participant_id
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch list of all registrations for participant from database")
    }

    pub async fn result_for_registration(
        &self,
        registration_id: Uuid,
    ) -> Result<Option<RegistrationResult>> {
        sqlx::query_as!(
            RegistrationResult,
            r#"
                SELECT
                    registration_id, disqualified, time_millis
                FROM registration_results
                WHERE registration_id = $1;
            "#,
            registration_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch registration result rom database")
    }

    /// Delete a registration.
    ///
    /// Beware that in order for not violating the database schema, there must
    /// no be any results for the registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration for which shall be
    ///   deleted.
    ///
    /// # Results:
    /// - `Ok(true)` - if the result has been deleted
    /// - `Ok(false)` - if the result did not exist
    /// - `Err(e)` - in case of an database error
    pub async fn delete_registration(&self, registration_id: Uuid) -> Result<bool> {
        let rows = sqlx::query!(
            r#"
                DELETE FROM registrations
                WHERE id = $1
            "#,
            registration_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete registration in database")?
        .rows_affected();

        Ok(rows > 0)
    }

    /// Delete a result of a registration.
    ///
    /// # Parameters:
    /// - `registration_id` - The id of the registration for which the result
    ///   shall be deleted.
    ///
    /// # Results:
    /// - `Ok(Some(()))` - if the result has been deleted
    /// - `Ok(None)` - if the result did not exist
    /// - `Err(e)` - in case of an database error
    pub async fn delete_result_for_registration(
        &self,
        registration_id: Uuid,
    ) -> Result<Option<()>> {
        let rows = sqlx::query!(
            r#"
                DELETE FROM registration_results
                WHERE registration_id = $1
            "#,
            registration_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete registration result in database")?
        .rows_affected();

        if rows > 0 {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    /// Create a new registration.
    ///
    /// The given participant and the competition must both exist, otherwise
    /// there will be an database error due to unfulfilled constraints.
    ///
    /// # Parameters:
    /// - `participant_id` - The id of the participant that registers
    /// - `competition_id` - The id of the competition to register for
    ///
    /// # Returns:
    /// - `Ok(registration_id)` - If a new registration has been created or
    ///    already exists.
    /// - `Err(e)` - In case of a database error
    pub async fn create_registration(
        &self,
        participant_id: Uuid,
        competition_id: Uuid,
    ) -> Result<Uuid> {
        let existing_registration_id = sqlx::query_scalar!(
            r#"
                SELECT
                    id AS "id!"
                FROM registrations
                WHERE
                    participant_id = $1 AND
                    competition_id = $2;
            "#,
            participant_id,
            competition_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to look for existing registration in database")?;

        if let Some(registration_id) = existing_registration_id {
            return Ok(registration_id);
        }

        let registration_id = sqlx::query_scalar!(
            r#"
                INSERT INTO registrations (
                    participant_id, competition_id
                ) VALUES (
                    $1, $2
                ) RETURNING id;
            "#,
            participant_id,
            competition_id
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to insert registration in database")?;

        Ok(registration_id)
    }

    pub async fn registration_by_id(&self, registration_id: Uuid) -> Result<Option<Registration>> {
        sqlx::query_as!(
            Registration,
            r#"
                SELECT
                    id, participant_id, competition_id
                FROM registrations
                WHERE id = $1;
            "#,
            registration_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch registrations by id from database")
    }

    pub async fn create_registration_result(
        &self,
        registration_id: Uuid,
        time_millis: i32,
        disqualified: bool,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO registration_results (
                    registration_id, time_millis, disqualified
                ) VALUES (
                    $1, $2, $3
                );
            "#,
            registration_id,
            time_millis,
            disqualified
        )
        .execute(&self.pool)
        .await
        .context("Failed to insert registration in database")?;

        Ok(())
    }
}
