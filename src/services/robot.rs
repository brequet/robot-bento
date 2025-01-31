use sqlx::PgPool;

use crate::repositories::robot::RobotRepository;

use super::{mappers::robot::RobotMapper, parser::TestRun};

pub struct RobotService;

impl RobotService {
    pub async fn ingest(
        pool: &PgPool,
        parsed_test_run: TestRun,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let test_run = RobotMapper::map_test_run_to_db(&parsed_test_run);
        match test_run {
            Ok(test_run) => RobotRepository::insert_test_run(pool, &test_run).await?,
            Err(e) => return Err(Box::new(e)),
        };
        Ok(())
    }
}
