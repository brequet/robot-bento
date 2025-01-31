use crate::models::robot::TestRunDB;
use sqlx::{query_file, PgPool};

pub struct RobotRepository;

impl RobotRepository {
    pub async fn insert_test_run(pool: &PgPool, test_run: &TestRunDB) -> Result<i32, sqlx::Error> {
        let result = query_file!(
            "./src/repositories/sql/robot/insert_test_run.sql",
            test_run.rpa,
            test_run.generator,
            test_run.generated_date,
            test_run.schema_version
        )
        .fetch_one(pool)
        .await?;

        Ok(result.id)
    }
}
