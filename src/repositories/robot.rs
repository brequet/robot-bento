use crate::models::robot::{ErrorDB, StatDB, StatTypeDB, TestRunDB};
use chrono::NaiveDateTime;
use sqlx::{query_file, query_file_as, PgPool};

struct TestRunDBPartial {
    pub id: Option<i32>,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
}

pub struct RobotRepository;

impl RobotRepository {
    pub async fn get_test_run_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<TestRunDB>, sqlx::Error> {
        let test_run = match query_file_as!(
            TestRunDBPartial,
            "./src/repositories/sql/robot/get_test_run_by_id.sql",
            id
        )
        .fetch_optional(pool)
        .await?
        {
            Some(r) => r,
            None => return Ok(None),
        };

        let statistics = query_file_as!(
            StatDB,
            "./src/repositories/sql/robot/get_test_run_statistics_by_test_run_id.sql",
            id
        )
        .fetch_all(pool)
        .await?;

        let errors = query_file_as!(
            ErrorDB,
            "./src/repositories/sql/robot/get_test_run_errors_by_test_run_id.sql",
            id
        )
        .fetch_all(pool)
        .await?;

        Ok(Some(TestRunDB {
            id: test_run.id,
            rpa: test_run.rpa,
            generator: test_run.generator,
            generated_date: test_run.generated_date,
            schema_version: test_run.schema_version,
            statistics,
            errors,
        }))
    }

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

        let test_run_id = result.id;

        RobotRepository::insert_statistics(pool, test_run_id, &test_run.statistics).await?;
        RobotRepository::insert_errors(pool, test_run_id, &test_run.errors).await?;

        Ok(test_run_id)
    }

    async fn insert_statistics(
        pool: &PgPool,
        test_run_id: i32,
        statistics: &Vec<StatDB>,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO test_run_statistics (test_run_id, stat_type, pass_count, fail_count, skip_count, identifier, name, text) "
        );

        query_builder.push_values(statistics, |mut b, stat| {
            b.push_bind(test_run_id)
                .push_bind(&stat.stat_type)
                .push_bind(stat.pass_count)
                .push_bind(stat.fail_count)
                .push_bind(stat.skip_count)
                .push_bind(&stat.identifier)
                .push_bind(&stat.name)
                .push_bind(&stat.text);
        });

        query_builder.build().execute(pool).await?;
        Ok(())
    }

    async fn insert_errors(
        pool: &PgPool,
        test_run_id: i32,
        errors: &Vec<ErrorDB>,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO test_run_errors (test_run_id, timestamp, level, content) ",
        );

        query_builder.push_values(errors, |mut b, error| {
            b.push_bind(test_run_id)
                .push_bind(&error.timestamp)
                .push_bind(&error.level)
                .push_bind(&error.content);
        });

        query_builder.build().execute(pool).await?;
        Ok(())
    }
}
