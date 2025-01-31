use chrono::NaiveDateTime;

use crate::models::robot::TestRunDB;
use crate::services::parser::TestRun;

pub struct RobotMapper;

impl RobotMapper {
    pub fn map_test_run_to_db(test_run: &TestRun) -> Result<TestRunDB, chrono::ParseError> {
        Ok(TestRunDB {
            id: None,
            rpa: test_run.rpa,
            generator: test_run.generator.clone(),
            generated_date: NaiveDateTime::parse_from_str(
                &test_run.generated_date.clone(),
                "%Y%m%d %H:%M:%S%.3f",
            )?,
            schema_version: test_run.schema_version.clone(),
        })
    }
}
