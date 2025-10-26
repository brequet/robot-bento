use std::{collections::HashMap, sync::Arc};

use crate::{
    models::{self, robot::{db::{ErrorDB, ProjectTestSummaryDB, RawJsonRecord, StatisticDB, SuiteDB, SuiteKeywordRecord, TestDB}, domain::{ProjectTestRunSummary, SavedTestRun, SuiteKeywords, TestRunError, TestRunStatistic, TestRunSuite, TestRunTest}}, robot_legacy::{ErrorDBLegacy, StatDBLegacy, SuiteDBLegacy, TestDBLegacy, TestRunDBLegacy}},
    services::parser::{self}};
use serde_json::Value;
use sqlx::{query_as, query_file, query_file_as, query_scalar, PgPool};
use crate::models::robot::db::StatisticTypeDB;

enum SuiteKeywordType {
    Setup,
    Teardown,
}

impl SuiteKeywordType {
    fn as_str(&self) -> &'static str {
        match self {
            SuiteKeywordType::Setup => "setup",
            SuiteKeywordType::Teardown => "teardown",
        }
    }
}

pub struct RobotRepository {
    pool: PgPool,
}

impl RobotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_latest_test_runs_summaries_for_projects(&self, project_ids: &Vec<i32>) -> Result<Vec<ProjectTestRunSummary>, sqlx::Error> {
        query_file_as!(
            ProjectTestSummaryDB,
            "./src/repositories/queries/robot/get_latest_test_runs_summaries_for_projects.sql",
            project_ids
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_latest_test_runs_summaries_for_projects failed: {:?}", e))
        .map(|test_run_summary| {
            test_run_summary
                .into_iter()
                .map(|test_run_summary| test_run_summary.into_summary())
                .collect()
        })
    }

    pub async fn get_test_runs_summaries_by_project_id(&self, project_id: i32) -> Result<Vec<ProjectTestRunSummary>, sqlx::Error> {
        query_file_as!(
            ProjectTestSummaryDB,
            "./src/repositories/queries/robot/get_test_runs_summaries_by_project_id.sql",
            project_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_test_runs_summaries_by_project_id failed: {:?}", e))
        .map(|test_run_summary| {
            test_run_summary
                .into_iter()
                .map(|test_run_summary| test_run_summary.into_summary())
                .collect()
        })
    }

    pub async fn get_test_run_by_id(
        &self,
        id: i32,
    ) -> Result<Option<SavedTestRun>, sqlx::Error> {
        let result = query_as!(
            models::robot::db::TestRunDB,
            r#"
            SELECT tr.id,
                tr.project_id,
                tr.rpa,
                tr.generator,
                tr.generated_date,
                tr.schema_version,
                tr.application_version,
                tr.imported_date
            FROM test_runs tr
            WHERE tr.id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_test_run_by_id failed: {:?}", e))?;
        
        let test_run = match result {
            Some(test_run_db) => {
                let suites = self.get_suites_by_test_run_id_and_parent_suite_id(id, None).await?;
                let statistics = self.get_test_run_statistics_by_test_run_id(id).await?;
                let errors = self.get_test_run_errors_by_test_run_id(id).await?;
                Some(test_run_db.to_test_run(suites, statistics, errors))
            }
            None => None,
        };
        
        Ok(test_run)
    }

    pub async fn is_sha1_already_inserted(&self, sha1: &str) -> Result<bool, sqlx::Error> {
        let is_inserted: Option<bool> = query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM test_runs WHERE sha1 = $1)",
            sha1
        )
        .fetch_one(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query is_sha1_already_inserted failed: {:?}", e))?;

        Ok(is_inserted.unwrap_or(true))
    }

    pub async fn insert_test_run(&self, test_run: &TestRunDBLegacy, project_id: i32) -> Result<i32, sqlx::Error> {
        let result = query_file!(
            "./src/repositories/queries/robot/insert_test_run.sql",
            project_id,
            test_run.rpa,
            test_run.generator,
            test_run.generated_date,
            test_run.schema_version,
            test_run.app_version,
            test_run.sha1
        )
        .fetch_one(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query insert_test_run failed: {:?}", e))?;

        let test_run_id = result.id;

        self.insert_suites( test_run_id, None, &test_run.suites).await?;
        self.insert_statistics( test_run_id, &test_run.statistics).await?;
        self.insert_errors( test_run_id, &test_run.errors).await?;

        Ok(test_run_id)
    }
    
    pub async fn get_suite_keywords_by_suite_id(
        &self,
        suite_id: i32
    ) -> Result<Option<SuiteKeywords>, sqlx::Error> {
        let keywords = query_as!(
            SuiteKeywordRecord,
            r#"--sql
            SELECT type as keyword_type,
                  value
            FROM suite_keywords
            WHERE suite_id = $1
            "#,
            suite_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_suite_keywords_by_suite_id failed: {:?}", e))?;

        if keywords.is_empty() {
            return Ok(None);
        }
    
        let map = keywords
            .into_iter()
            .map(|k| (k.keyword_type, k.value))
            .collect::<HashMap<_, _>>();
    
        Ok(Some(SuiteKeywords {
            keywords: Arc::new(map),
        }))
    }

    pub async fn get_test_keywords_by_test_id(
        &self,
        test_id: i32
    ) -> Result<Option<Value>, sqlx::Error> {
        query_as!(
            RawJsonRecord,
            r#"--sql
                SELECT value
                FROM test_keywords
                WHERE test_id = $1
                LIMIT 1;
            "#,
            test_id
        )
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_test_keywords_by_test_id failed: {:?}", e))
        .map(|raw_json| raw_json.map(|r| r.value))
    }

    async fn get_suites_by_test_run_id_and_parent_suite_id(
        &self,
        test_run_id: i32,
        parent_suite_id: Option<i32>,
    ) -> Result<Vec<TestRunSuite>, sqlx::Error> {
        Box::pin(async move {

        let mut query_builder = sqlx::QueryBuilder::new(
            r#"
            SELECT s.id,
                   s.name,
                   s.source,
                   s.status,
                   s.start_time,
                   s.end_time,
                   s.identifier,
                   s.doc
            FROM suites s
            WHERE s.test_run_id = 
            "#
        );

        query_builder.push_bind(test_run_id);

        if let Some(parent_suite_id) = parent_suite_id {
            query_builder.push(" AND s.parent_suite_id = ").push_bind(parent_suite_id);
        } else {
            query_builder.push(" AND s.parent_suite_id IS NULL");
        }

        query_builder.push(" ORDER BY s.start_time ASC");
        
        let suites: Vec<SuiteDB> = query_builder
        .build_query_as()
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_suites_by_test_run_id_and_parent_suite_id failed: {:?}", e))?;

        let mut suite_db_list = Vec::new();
        for suite in suites {
            // let (setup_keyword, teardown_keyword) = self.get_suite_keywords_by_suite_id( suite.id.unwrap()).await?;
            let suites = self.get_suites_by_test_run_id_and_parent_suite_id( test_run_id, Some(suite.id)).await?;
            let tests = self.get_tests_by_suite_id( suite.id).await?;

            suite_db_list.push(suite.to_test_run_suite(suites, tests));
        }
        Ok(suite_db_list)
    }).await
    }

    async fn get_tests_by_suite_id(
        &self,
        suite_id: i32,
    ) -> Result<Vec<TestRunTest>, sqlx::Error> {
        let rows = query_as!(
            TestDB,
            r#"--sql
            SELECT 
                t.id,
                t.identifier,
                t.name,
                t.status,
                t.start_time,
                t.end_time,
                t.line,
                t.doc,
                t.timeout,
                tt.value as "tag?"
            FROM tests t
            LEFT JOIN test_tags tt ON t.id = tt.test_id
            WHERE t.suite_id = $1
            ORDER BY t.start_time ASC, tt.value ASC
            "#,
            suite_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_tests_by_suite_id failed for suite_id={}: {:?}", suite_id, e))?;
        
        let mut tests_map: HashMap<i32, (&TestDB, Vec<String>)> = HashMap::new();
        
        for row in &rows {
            tests_map
                .entry(row.id)
                .and_modify(|(_, tags)| {
                    if let Some(tag) = &row.tag {
                        tags.push(tag.clone());
                    }
                })
                .or_insert_with(|| (row, row.tag.clone().into_iter().collect()));
        }

        Ok(tests_map
            .into_values()
            .map(|(&ref test, tags)| test.to_(tags))
            .collect())
    }

    async fn get_test_run_statistics_by_test_run_id(
        &self,
        test_run_id: i32,
    ) -> Result<Vec<TestRunStatistic>, sqlx::Error> {
        query_as!(
            StatisticDB,
            r#"--sql
            SELECT stats.id,
                stats.stat_type as "stat_type: StatisticTypeDB",
                stats.pass_count,
                stats.fail_count,
                stats.skip_count,
                stats.identifier,
                stats.name,
                stats.text
            FROM test_run_statistics stats
            WHERE stats.test_run_id = $1;
            "#,
            test_run_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| {
            tracing::error!(
                "Query get_test_run_statistics_by_test_run_id failed: {:?}",
                e
            )
        })
        .map(|stats| stats.iter().map(StatisticDB::to_test_run_statistic).collect())
    }

    async fn get_test_run_errors_by_test_run_id(
        &self,
        test_run_id: i32,
    ) -> Result<Vec<TestRunError>, sqlx::Error> {
        query_as!(
            ErrorDB,
            r#"--sql
            SELECT errors.id,
                errors.timestamp,
                errors.level,
                errors.content
            FROM test_run_errors errors
            WHERE errors.test_run_id = $1;
            "#,
            test_run_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| {
            tracing::error!("Query get_test_run_errors_by_test_run_id failed: {:?}", e)
        })
        .map(|errors| errors.iter().map(ErrorDB::to_test_run_error).collect())
    }

    async fn insert_suites(
        &self,
        test_run_id: i32,
        parent_suite_id: Option<i32>,
        suites: &Vec<SuiteDBLegacy>,
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

            let ids: Vec<(i32,)> = query_builder.build_query_as().fetch_all(&self.pool).await
            .inspect_err(|e| tracing::error!("Query insert_suites failed: {:?}", e))?;

            for (suite, (id,)) in suites.iter().zip(ids) {
                if !suite.suites.is_empty() {
                    self.insert_suites(test_run_id, Some(id), &suite.suites).await?;
                }
                if !suite.tests.is_empty() {
                    self.insert_tests(id, &suite.tests).await?;
                }
                if let Some(setup_kw) = &suite.setup_keyword {
                    self.insert_suite_keyword(id, SuiteKeywordType::Setup, setup_kw.clone()).await?;
                }
                if let Some(teardown_kw) = &suite.teardown_keyword {
                    self.insert_suite_keyword(id, SuiteKeywordType::Teardown, teardown_kw.clone()).await?;
                }
            }

            Ok(())
        }).await
    }

    async fn insert_suite_keyword(
        &self,
        suite_id: i32,
        keyword_type: SuiteKeywordType,
        keyword: parser::Keyword,
    ) -> Result<(), sqlx::Error> {
        let json_keyword = match serde_json::to_value(&keyword) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!("Failed to serialize keyword: {:?}", e);
                return Err(sqlx::Error::Protocol("Failed to serialize keyword".into()));
            }
        };

        query_file!(
            "./src/repositories/queries/robot/insert_suite_keyword.sql",
            suite_id,
            keyword_type.as_str(),
            json_keyword
        )
        .execute(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query insert_suite_keyword failed: {:?}", e))?;

        Ok(())
    }

    async fn insert_tests(
        &self,
        suite_id: i32,
        tests: &Vec<TestDBLegacy>,
    ) -> Result<(), sqlx::Error> {
        if tests.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO tests (suite_id, identifier, name, status, start_time, end_time, line, doc, timeout) ",
        );

        query_builder.push_values(tests, |mut b, test| {
            b.push_bind(suite_id)
                .push_bind(&test.identifier)
                .push_bind(&test.name)
                .push_bind(&test.status)
                .push_bind(&test.start_time)
                .push_bind(&test.end_time)
                .push_bind(&test.line)
                .push_bind(&test.doc)
                .push_bind(&test.timeout);
        });

        query_builder.push(" RETURNING id");

        let ids: Vec<(i32,)> = query_builder.build_query_as().fetch_all(&self.pool).await
        .inspect_err(|e| tracing::error!("Query insert_tests failed: {:?}", e))?;

        for (test, (id,)) in tests.iter().zip(ids) {
            if !test.tags.is_empty() {
                self.insert_test_tags(id, &test.tags).await?
            }
            if !test.keywords.is_empty() {
                self.insert_test_keywords(id, &test.keywords).await?
            }
        }

        Ok(())  
    }
    
    async fn insert_test_tags(
        &self,
        test_id: i32,
        tags: &Vec<String>,
    ) -> Result<(), sqlx::Error> {
        if tags.is_empty() {
            return Ok(());
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO test_tags (test_id, value) ",
        );

        query_builder.push_values(tags, |mut b, tag| {
            b.push_bind(test_id)
                .push_bind(tag);
        });

        query_builder
            .build()
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_test_tags failed: {:?}", e))?;
        Ok(())
    }

    async fn insert_test_keywords(
        &self,
        test_id: i32,
        keywords: &Vec<parser::BaseBody>,
    ) -> Result<(), sqlx::Error> {
        let json_keywords = match serde_json::to_value(&keywords) {
            Ok(json) => json,
            Err(e) => {
                tracing::error!("Failed to serialize keyword: {:?}", e);
                return Err(sqlx::Error::Protocol("Failed to serialize keyword".into()));
            }
        };

        query_file!(
            "./src/repositories/queries/robot/insert_test_keywords.sql",
            test_id,
            json_keywords
        )
        .execute(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query insert_test_keywords failed: {:?}", e))?;

        Ok(())
    }

    async fn insert_statistics(
        &self,
        test_run_id: i32,
        statistics: &Vec<StatDBLegacy>,
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
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_statistics failed: {:?}", e))?;
        Ok(())
    }

    async fn insert_errors(
        &self,
        test_run_id: i32,
        errors: &Vec<ErrorDBLegacy>,
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
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_errors failed: {:?}", e))?;
        Ok(())
    }
}
