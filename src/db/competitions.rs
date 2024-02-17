use anyhow::{Context as _, Result};
use uuid::Uuid;

pub struct Competition {
    pub id: Uuid,
    pub gender: super::Gender,
    pub stroke: super::Stroke,
    pub distance: i32,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
}

#[derive(Clone)]
pub struct Repository {
    pool: super::DatabasePool,
}

impl Repository {
    pub fn new(pool: super::DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn competition_by_id(&self, competition_id: Uuid) -> Result<Option<Competition>> {
        sqlx::query_as!(
            Competition,
            r#"
                SELECT
                    id, gender AS "gender: _", stroke AS "stroke: _", distance,
                    min_age, max_age
                FROM competitions
                WHERE id = $1;
            "#,
            competition_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch list of all registrations for participant from database")
    }
}
