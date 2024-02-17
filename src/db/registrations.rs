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
}
