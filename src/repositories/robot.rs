use crate::models::robot::{ErrorDB, StatDB, StatTypeDB, SuiteDB, TestRunDB};
use chrono::NaiveDateTime;
use sqlx::{query_file, query_file_as, PgPool};

struct TestRunDBPartial {
    pub id: Option<i32>,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
}
struct SuiteDBPartial {
    pub id: Option<i32>,
    pub name: String,
    pub source: Option<String>,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub identifier: String,
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
        .await
        .inspect_err(|e| tracing::error!("Query get_test_run_by_id failed: {:?}", e))?
        {
            Some(r) => r,
            None => return Ok(None),
        };

        let suites = RobotRepository::get_suites_by_test_run_id(pool, id).await?;
        let statistics = RobotRepository::get_test_run_statistics_by_test_run_id(pool, id).await?;
        let errors = RobotRepository::get_test_run_errors_by_test_run_id(pool, id).await?;

        Ok(Some(TestRunDB {
            id: test_run.id,
            rpa: test_run.rpa,
            generator: test_run.generator,
            generated_date: test_run.generated_date,
            schema_version: test_run.schema_version,
            suites,
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
        .await
        .inspect_err(|e| tracing::error!("Query insert_test_run failed: {:?}", e))?;

        let test_run_id = result.id;

        RobotRepository::insert_suites(pool, test_run_id, None, &test_run.suites).await?;
        RobotRepository::insert_statistics(pool, test_run_id, &test_run.statistics).await?;
        RobotRepository::insert_errors(pool, test_run_id, &test_run.errors).await?;

        Ok(test_run_id)
    }

    async fn get_suites_by_test_run_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        test_run_id: i32,
    ) -> Result<Vec<SuiteDB>, sqlx::Error> {
        let suites = query_file_as!(
            SuiteDBPartial,
            "./src/repositories/sql/robot/get_suites_by_test_run_id.sql",
            test_run_id
        )
        .fetch_all(pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_suites_by_test_run_id failed: {:?}", e))?;
        Ok(suites
            .iter()
            .map(|suite| SuiteDB {
                id: suite.id,
                name: suite.name.clone(),
                source: suite.source.clone(),
                status: suite.status.clone(),
                start_time: suite.start_time,
                end_time: suite.end_time,
                doc: suite.doc.clone(),
                identifier: suite.identifier.clone(),
                suites: Vec::new(),
            })
            .collect())
    }

    async fn get_test_run_statistics_by_test_run_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        test_run_id: i32,
    ) -> Result<Vec<StatDB>, sqlx::Error> {
        let statistics = query_file_as!(
            StatDB,
            "./src/repositories/sql/robot/get_test_run_statistics_by_test_run_id.sql",
            test_run_id
        )
        .fetch_all(pool)
        .await
        .inspect_err(|e| {
            tracing::error!(
                "Query get_test_run_statistics_by_test_run_id failed: {:?}",
                e
            )
        })?;
        Ok(statistics)
    }

    async fn get_test_run_errors_by_test_run_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        test_run_id: i32,
    ) -> Result<Vec<ErrorDB>, sqlx::Error> {
        let errors = query_file_as!(
            ErrorDB,
            "./src/repositories/sql/robot/get_test_run_errors_by_test_run_id.sql",
            test_run_id
        )
        .fetch_all(pool)
        .await
        .inspect_err(|e| {
            tracing::error!("Query get_test_run_errors_by_test_run_id failed: {:?}", e)
        })?;
        Ok(errors)
    }

    async fn insert_suites(
        pool: &PgPool,
        test_run_id: i32,
        parent_suite_id: Option<i32>,
        suites: &Vec<SuiteDB>,
    ) -> Result<(), sqlx::Error> {
        Box::pin(async move {
            if suites.is_empty() {
                return Ok(());
            }
            
            let mut query_builder = sqlx::QueryBuilder::new(
                "INSERT INTO suites (test_run_id, name, source, status, start_time, end_time, identifier, parent_suite_id, doc) ",
            );

            query_builder.push_values(suites, |mut b, suite| {
                b.push_bind(test_run_id)
                    .push_bind(&suite.name)
                    .push_bind(&suite.source)
                    .push_bind(&suite.status)
                    .push_bind(&suite.start_time)
                    .push_bind(&suite.end_time)
                    .push_bind(&suite.identifier)
                    .push_bind(parent_suite_id)
                    .push_bind(&suite.doc);
            });

            query_builder.push(" RETURNING id");

            let ids: Vec<(i32,)> = query_builder.build_query_as().fetch_all(pool).await
            .inspect_err(|e| tracing::error!("Query insert_suites failed: {:?}", e))?;

            for (suite, (id,)) in suites.iter().zip(ids) {
                if !suite.suites.is_empty() {
                    RobotRepository::insert_suites(pool, test_run_id, Some(id), &suite.suites).await?;
                }
            }

            Ok(())
        }).await
    }

    async fn insert_statistics(
        pool: &PgPool,
        test_run_id: i32,
        statistics: &Vec<StatDB>,
    ) -> Result<(), sqlx::Error> {
        if statistics.is_empty() {
            return Ok(());
        }

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

        query_builder
            .build()
            .execute(pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_statistics failed: {:?}", e))?;
        Ok(())
    }

    async fn insert_errors(
        pool: &PgPool,
        test_run_id: i32,
        errors: &Vec<ErrorDB>,
    ) -> Result<(), sqlx::Error> {
        if errors.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO test_run_errors (test_run_id, timestamp, level, content) ",
        );

        query_builder.push_values(errors, |mut b, error| {
            b.push_bind(test_run_id)
                .push_bind(&error.timestamp)
                .push_bind(&error.level)
                .push_bind(&error.content);
        });

        query_builder
            .build()
            .execute(pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_errors failed: {:?}", e))?;
        Ok(())
    }
}
