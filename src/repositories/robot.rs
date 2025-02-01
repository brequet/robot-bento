use crate::models::robot::{StatDB, StatTypeDB, TestRunDB};
use sqlx::{query_file, PgPool};

pub struct RobotRepository;

impl RobotRepository {
    pub async fn get_test_run_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<Option<TestRunDB>, sqlx::Error> {
        let rows = query_file!("./src/repositories/sql/robot/get_test_run_by_id.sql", id)
            .fetch_all(pool)
            .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let first_row = &rows[0];
        let test_run = TestRunDB {
            id: Some(first_row.id),
            rpa: first_row.rpa,
            generator: first_row.generator.clone(),
            generated_date: first_row.generated_date,
            schema_version: first_row.schema_version.clone(),
            statistics: rows
                .iter()
                .filter_map(|row| {
                    Some(StatDB {
                        id: Some(row.stat_id),
                        stat_type: row.stat_type.clone(),
                        pass_count: row.pass_count,
                        fail_count: row.fail_count,
                        skip_count: row.skip_count,
                        identifier: row.identifier.clone(),
                        name: row.name.clone(),
                        text: row.text.clone(),
                    })
                })
                .collect(),
        };

        Ok(Some(test_run))
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

        Ok(test_run_id)
    }

    async fn insert_statistics(
        pool: &PgPool,
        test_run_id: i32,
        statistics: &Vec<StatDB>,
    ) -> Result<(), sqlx::Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO statistics (test_run_id, stat_type, pass_count, fail_count, skip_count, identifier, name, text) "
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
}
