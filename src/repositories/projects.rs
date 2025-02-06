use chrono::NaiveDateTime;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct ProjectDB {
    pub id: Option<i32>,
    pub name: String,
    pub create_date: Option<NaiveDateTime>,
}

pub struct ProjectsRepository {
    pool: PgPool,
}

impl ProjectsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_projects(&self) -> Result<Vec<ProjectDB>, sqlx::Error> {
        sqlx::query_as!(
            ProjectDB,
            r#"--sql
            SELECT id, name, create_date
            FROM projects
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_projects failed: {:?}", e))
    }

    pub async fn get_project_by_id(
        &self,
        project_id: i32,
    ) -> Result<Option<ProjectDB>, sqlx::Error> {
        sqlx::query_as!(
            ProjectDB,
            r#"--sql
            SELECT id, name, create_date
            FROM projects
            WHERE id = $1
            "#,
            project_id
        )
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_project_by_id failed: {:?}", e))
    }

    pub async fn get_project_id_by_name(
        &self,
        project_name: &str,
    ) -> Result<Option<i32>, sqlx::Error> {
        sqlx::query_scalar!(
            r#"--sql
            SELECT id
            FROM projects
            WHERE name = $1
            "#,
            project_name
        )
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_project_id_by_name failed: {:?}", e))
    }

    pub async fn insert_project(&self, project: ProjectDB) -> Result<i32, sqlx::Error> {
        sqlx::query_scalar!(
            r#"--sql
            INSERT INTO projects (name)
            VALUES ($1)
            RETURNING id
            "#,
            project.name
        )
        .fetch_one(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query insert_project failed: {:?}", e))
    }
}
