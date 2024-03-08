use anyhow::{Context as _, Result};
use uuid::Uuid;

pub struct Group {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone)]
pub struct Repository {
    pool: super::DatabasePool,
}

impl Repository {
    pub fn new(pool: super::DatabasePool) -> Self {
        Self { pool }
    }

    pub async fn create_group(&self, name: String) -> Result<Uuid> {
        sqlx::query_scalar!(
            r#"
                INSERT INTO groups (
                    name
                ) VALUES (
                    $1
                ) RETURNING id;
            "#,
            name,
        )
        .fetch_one(&self.pool)
        .await
        .context("Failed to execute INSERT INTO query")
    }

    pub async fn all_groups(&self) -> Result<Vec<Group>> {
        sqlx::query_as!(
            Group,
            r#"
                SELECT
                    id, name
                FROM groups;
            "#
        )
        .fetch_all(&self.pool)
        .await
        .context("Failed to fetch list of all groups from database")
    }

    pub async fn group_by_id(&self, group_id: Uuid) -> Result<Option<Group>> {
        sqlx::query_as!(
            Group,
            r#"
                SELECT
                    id, name
                FROM groups
                WHERE id = $1;
            "#,
            group_id
        )
        .fetch_optional(&self.pool)
        .await
        .context("Failed to fetch group by id from database")
    }

    pub async fn delete_group(&self, group_id: Uuid) -> Result<Option<()>> {
        let rows = sqlx::query!(
            r#"
                DELETE FROM groups
                WHERE id = $1
            "#,
            group_id
        )
        .execute(&self.pool)
        .await
        .context("Failed to delete group in database")?
        .rows_affected();

        if rows > 0 {
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }
}
