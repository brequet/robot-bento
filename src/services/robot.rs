use sqlx::PgPool;
use tracing::info;

use crate::{models::robot::TestRunDB, repositories::robot::RobotRepository};

use super::{mappers, parser::TestRun};

pub struct RobotService;

impl RobotService {
    pub async fn save_test_run(
        pool: &PgPool,
        parsed_test_run: TestRun,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_run = mappers::robot::map_test_run(&parsed_test_run);
        match test_run {
            Ok(test_run) => match RobotRepository::insert_test_run(pool, &test_run).await {
                Ok(test_run_id) => {
                    info!("Saved test run, id: {}", test_run_id);
                }
                Err(e) => return Err(Box::new(e)),
            },
            Err(e) => return Err(Box::new(e)),
        };
        Ok(())
    }

    pub async fn get_all_test_runs(
        pool: &PgPool,
    ) -> Result<Vec<TestRunDB>, Box<dyn std::error::Error>> {
        let test_runs = RobotRepository::get_all_test_runs(pool).await?;
        Ok(test_runs)
    }

    pub async fn get_test_run_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<TestRunDB>, Box<dyn std::error::Error>> {
        let test_run = RobotRepository::get_test_run_by_id(pool, id).await?;
        Ok(test_run)
    }
}
