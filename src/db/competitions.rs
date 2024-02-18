use anyhow::{Context as _, Result};
use uuid::Uuid;

use super::{Gender, Stroke};

pub struct Competition {
    pub id: Uuid,
    pub gender: super::Gender,
    pub stroke: super::Stroke,
    pub distance: i32,
}

#[derive(Clone)]
pub struct Repository {
    pool: super::DatabasePool,
}

impl Repository {
    pub fn new(pool: super::DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn search_competition(
        &self,
        gender: Option<Gender>,
        stroke: Option<Stroke>,
        distance: Option<i32>,
    ) -> Result<Vec<Competition>> {
        sqlx::query_as!(
            Competition,
            r#"
                SELECT
                    id, gender AS "gender: _", stroke AS "stroke: _", distance
                FROM competitions
                WHERE
                    (gender = $1 OR $1 IS NULL) AND
                    (stroke = $2 OR $2 IS NULL) AND
                    (distance = $3 OR $3 IS NULL);
            "#,
            gender as Option<Gender>,
            stroke as Option<Stroke>,
            distance
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch search results of competitions from database")
    }

    pub async fn all_competitions(&self) -> Result<Vec<Competition>> {
        sqlx::query_as!(
            Competition,
            r#"
                SELECT
                    id, gender AS "gender: _", stroke AS "stroke: _", distance
                FROM competitions;
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch list of all competitions from database")
    }

    pub async fn competition_by_id(&self, competition_id: Uuid) -> Result<Option<Competition>> {
        sqlx::query_as!(
            Competition,
            r#"
                SELECT
                    id, gender AS "gender: _", stroke AS "stroke: _", distance
                FROM competitions
                WHERE id = $1;
            "#,
            competition_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch competition by id from database")
    }
}
