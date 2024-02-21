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

    pub async fn create_competition(
        &self,
        gender: Gender,
        stroke: Stroke,
        distance: i32,
    ) -> Result<Uuid> {
        sqlx::query_scalar!(
            r#"
                INSERT INTO competitions (
                    gender, stroke, distance
                ) VALUES (
                    $1, $2, $3
                ) RETURNING id;
            "#,
            gender as Gender,
            stroke as Stroke,
            distance,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to execute INSERT INTO query")
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

    pub async fn delete_competition(&self, competition_id: Uuid) -> Result<Option<()>> {
        let rows = sqlx::query!(
            r#"
                DELETE FROM competitions
                WHERE id = $1
            "#,
            competition_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete competition in database")?
        .rows_affected();

        if rows > 0 {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
}
