use sqlx::PgPool;
use tracing::{info, warn};

use crate::{models::robot::TestRunDB, repositories::robot::RobotRepository};

use super::{mappers, parser::TestRun};

pub struct TestRunMetadata {
    pub app_name: String,
    pub app_version: String,
}

pub struct RobotService;

impl RobotService {
    pub async fn save_test_run(
        pool: &PgPool,
        parsed_test_run: TestRun,
        metadata: TestRunMetadata,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_sha1 = parsed_test_run.sha1.as_ref().unwrap();
        if RobotRepository::is_sha1_already_inserted(pool, file_sha1).await? {
            warn!("Test run with sha1 {} already exists", &file_sha1);
            Err("Test run already imported")?;
        }

        let test_run = mappers::robot::map_test_run(&parsed_test_run, &metadata);
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
