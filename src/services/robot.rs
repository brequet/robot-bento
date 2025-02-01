use sqlx::PgPool;

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
                    println!("Ingested test run with id: {}", test_run_id); // TODO: better logging
                }
                Err(e) => return Err(Box::new(e)),
            },
            Err(e) => return Err(Box::new(e)),
        };
        Ok(())
    }

    pub async fn get_test_run_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<TestRunDB>, Box<dyn std::error::Error>> {
        let test_run = RobotRepository::get_test_run_by_id(pool, id).await?;
        Ok(test_run)
    }
}
