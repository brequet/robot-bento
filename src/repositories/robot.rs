use crate::{
    models::{robot::{db::ProjectTestSummaryDB, domain::ProjectLatestTestRunSummary}, robot_legacy::{ErrorDB, StatDB, StatTypeDB, SuiteDB, TestDB, TestRunDB}},
    services::parser::{self, Keyword}};
use chrono::NaiveDateTime;
use sqlx::{query_file, query_file_as, query_scalar, PgPool};

#[derive(sqlx::FromRow)]
struct TestRunDBPartial {
    pub id: Option<i32>,
    pub rpa: bool,
    pub generator: String,
    pub generated_date: NaiveDateTime,
    pub schema_version: String,
    pub application_version: String,
    pub sha1: String,
    pub imported_date: NaiveDateTime,
}

#[derive(sqlx::FromRow)]
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

#[derive(sqlx::FromRow)]
pub struct TestDBPartial {
    pub id: Option<i32>,
    pub name: String,
    pub line: i32,
    pub identifier: String,
    pub status: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub doc: Option<String>,
    pub timeout: Option<String>,
}

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

    pub async fn get_test_summaries(&self, project_ids: &Vec<i32>) -> Result<Vec<ProjectLatestTestRunSummary>, sqlx::Error> {
        query_file_as!(
            ProjectTestSummaryDB,
            "./src/repositories/queries/robot/get_test_summaries.sql",
            project_ids
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_test_summaries failed: {:?}", e))
        .map(|test_run_summary| {
            test_run_summary
                .into_iter()
                .map(|test_run_summary| test_run_summary.into_summary())
                .collect()
        })
    }

    pub async fn get_all_test_runs(&self) -> Result<Vec<TestRunDB>, sqlx::Error> {
        let test_runs = query_file_as!(
            TestRunDBPartial,
            "./src/repositories/queries/robot/get_all_test_runs.sql",
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_all_test_runs failed: {:?}", e))?;

        let mut test_run_dbs = Vec::new();
        for test_run in test_runs {
            let suites = self.get_suites_by_test_run_id_and_parent_suite_id( test_run.id.unwrap(), None).await?;
            let statistics = self.get_test_run_statistics_by_test_run_id( test_run.id.unwrap()).await?;
            let errors = self.get_test_run_errors_by_test_run_id( test_run.id.unwrap()).await?;
    
            test_run_dbs.push({TestRunDB {
                id: test_run.id,
                rpa: test_run.rpa,
                generator: test_run.generator,
                generated_date: test_run.generated_date,
                schema_version: test_run.schema_version,
                app_name: "".to_string(),
                app_version: test_run.application_version,
                sha1: test_run.sha1,
                imported_date: Some(test_run.imported_date),
                suites,
                statistics,
                errors,
            }})
        }

        Ok(test_run_dbs)
    }

    pub async fn get_all_test_runs_by_project_id(&self, project_id: i32) -> Result<Vec<TestRunDB>, sqlx::Error> {
        let test_runs = query_file_as!(
            TestRunDBPartial,
            "./src/repositories/queries/robot/get_all_test_runs_by_project_id.sql",
            project_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_all_test_runs_by_project_id failed: {:?}", e))?;

        let mut test_run_dbs = Vec::new();
        for test_run in test_runs {
            let suites = self.get_suites_by_test_run_id_and_parent_suite_id( test_run.id.unwrap(), None).await?;
            let statistics = self.get_test_run_statistics_by_test_run_id( test_run.id.unwrap()).await?;
            let errors = self.get_test_run_errors_by_test_run_id( test_run.id.unwrap()).await?;
    
            test_run_dbs.push({TestRunDB {
                id: test_run.id,
                rpa: test_run.rpa,
                generator: test_run.generator,
                generated_date: test_run.generated_date,
                schema_version: test_run.schema_version,
                app_name: "".to_string(),
                app_version: test_run.application_version,
                sha1: test_run.sha1,
                imported_date: Some(test_run.imported_date),
                suites,
                statistics,
                errors,
            }})
        }

        Ok(test_run_dbs)
    }
    
    pub async fn get_test_run_by_id(
        &self,
        id: i32,
    ) -> Result<Option<TestRunDB>, sqlx::Error> {
        let test_run = match query_file_as!(
            TestRunDBPartial,
            "./src/repositories/queries/robot/get_test_run_by_id.sql",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_test_run_by_id failed: {:?}", e))?
        {
            Some(r) => r,
            None => return Ok(None),
        };

        let suites = self.get_suites_by_test_run_id_and_parent_suite_id( id, None).await?;
        let statistics = self.get_test_run_statistics_by_test_run_id( id).await?;
        let errors = self.get_test_run_errors_by_test_run_id( id).await?;

        Ok(Some(TestRunDB {
            id: test_run.id,
            rpa: test_run.rpa,
            generator: test_run.generator,
            generated_date: test_run.generated_date,
            schema_version: test_run.schema_version,
            app_name: "".to_string(),
            app_version: test_run.application_version,
            sha1: test_run.sha1,
            imported_date: Some(test_run.imported_date),
            suites,
            statistics,
            errors,
        }))
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

    pub async fn insert_test_run(&self, test_run: &TestRunDB, project_id: i32) -> Result<i32, sqlx::Error> {
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

    async fn get_suites_by_test_run_id_and_parent_suite_id(
        &self,
        test_run_id: i32,
        parent_suite_id: Option<i32>,
    ) -> Result<Vec<SuiteDB>, sqlx::Error> {
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
        
        let suites: Vec<SuiteDBPartial> = query_builder
        .build_query_as()
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_suites_by_test_run_id_and_parent_suite_id failed: {:?}", e))?;

        let mut suite_dbs = Vec::new();
        for suite in suites {
            let (setup_keyword, teardown_keyword) = self.get_suite_keywords_by_suite_id( suite.id.unwrap()).await?;
            let suites = self.get_suites_by_test_run_id_and_parent_suite_id( test_run_id, suite.id).await?;
            let tests = self.get_tests_by_suite_id( suite.id.unwrap()).await?;
            suite_dbs.push(SuiteDB {
                id: suite.id,
                name: suite.name.clone(),
                source: suite.source.clone(),
                status: suite.status.clone(),
                start_time: suite.start_time,
                end_time: suite.end_time,
                doc: suite.doc.clone(),
                identifier: suite.identifier.clone(),
                setup_keyword,
                suites,
                tests,
                teardown_keyword
            });
        }
        Ok(suite_dbs)
    }).await
    }

    async fn get_suite_keywords_by_suite_id(
        &self,
        suite_id: i32
    ) -> Result<(Option<parser::Keyword>, Option<parser::Keyword>), sqlx::Error> {
        let keywords = sqlx::query!(
            "SELECT type as keyword_type, value as keyword_value FROM suite_keywords WHERE suite_id = $1",
            suite_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_suite_keywords_by_suite_id failed: {:?}", e))?;

        Ok((
            keywords
                .iter()
                .find(|k| k.keyword_type == "setup")
                .map(|k| serde_json::from_value::<Keyword>(k.keyword_value.clone()).unwrap()),
            keywords
                .iter()
                .find(|k| k.keyword_type == "teardown")
                .map(|k| serde_json::from_value::<Keyword>(k.keyword_value.clone()).unwrap())
        ))
    }

    async fn get_tests_by_suite_id(
        &self,
        suite_id: i32,
    ) -> Result<Vec<TestDB>, sqlx::Error> {
        let tests = query_file_as!(
            TestDBPartial,
            "./src/repositories/queries/robot/get_tests_by_suite_id.sql",
            suite_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_tests_by_suite_id failed: {:?}", e))?;

        let mut tests_dbs = Vec::new();
        for test in tests {
            let tags = self.get_tags_by_test_id( test.id.unwrap()).await?;
            tests_dbs.push(TestDB {
                id: test.id,
                name: test.name.clone(),
                line: test.line,
                identifier: test.identifier.clone(),
                status: test.status.clone(),
                start_time: test.start_time,
                end_time: test.end_time,
                doc: test.doc.clone(),
                timeout: test.timeout.clone(),
                tags,
                keywords: Vec::new(), // TODO: option to fill it ? else its too much everytime
            })
        }

     Ok(tests_dbs)
    }

    async fn get_tags_by_test_id(
        &self,
        test_id: i32,
    ) -> Result<Vec<String>, sqlx::Error> {
        let tags = query_scalar!(
            "SELECT value FROM test_tags WHERE test_id = $1",
            test_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!("Query get_tags_by_test_id failed: {:?}", e))?;

        Ok(tags)
    }

    async fn get_test_run_statistics_by_test_run_id(
        &self,
        test_run_id: i32,
    ) -> Result<Vec<StatDB>, sqlx::Error> {
        let statistics = query_file_as!(
            StatDB,
            "./src/repositories/queries/robot/get_test_run_statistics_by_test_run_id.sql",
            test_run_id
        )
        .fetch_all(&self.pool)
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
        &self,
        test_run_id: i32,
    ) -> Result<Vec<ErrorDB>, sqlx::Error> {
        let errors = query_file_as!(
            ErrorDB,
            "./src/repositories/queries/robot/get_test_run_errors_by_test_run_id.sql",
            test_run_id
        )
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| {
            tracing::error!("Query get_test_run_errors_by_test_run_id failed: {:?}", e)
        })?;
        Ok(errors)
    }

    async fn insert_suites(
        &self,
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
        tests: &Vec<TestDB>,
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
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_statistics failed: {:?}", e))?;
        Ok(())
    }

    async fn insert_errors(
        &self,
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
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!("Query insert_errors failed: {:?}", e))?;
        Ok(())
    }
}
