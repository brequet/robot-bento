use chrono::{NaiveDateTime, ParseResult};

use crate::models::robot_legacy::{
    ErrorDBLegacy, StatDBLegacy, StatTypeDB, SuiteDBLegacy, TestDBLegacy, TestRunDBLegacy,
};
use crate::services::{self, parser};

pub fn map_test_run(
    test_run: &parser::TestRun,
    metadata: &services::robot::TestRunMetadata,
) -> Result<TestRunDBLegacy, chrono::ParseError> {
    Ok(TestRunDBLegacy {
        id: None,
        imported_date: None,
        rpa: test_run.rpa,
        generator: test_run.generator.clone(),
        generated_date: map_timestamp(&test_run.generated_date.clone())?,
        schema_version: test_run.schema_version.clone(),
        sha1: test_run.sha1.clone(),
        app_name: metadata.app_name.clone(),
        app_version: metadata.app_version.clone(),
        suites: map_suites(&test_run.suites),
        statistics: map_statistics(&test_run.statistics),
        errors: map_errors(&test_run.errors),
    })
}

fn map_suites(suites: &Vec<parser::Suite>) -> Vec<SuiteDBLegacy> {
    suites.iter().map(|suite| map_suite(suite)).collect()
}

fn map_suite(suite: &parser::Suite) -> SuiteDBLegacy {
    let setup_keyword = match &suite.children.first() {
        Some(parser::SuiteChildren::Keyword(keyword)) => Some(keyword.clone()),
        _ => None,
    };
    let teardown_keyword = if suite.children.len() > 1 {
        match &suite.children.last() {
            Some(parser::SuiteChildren::Keyword(keyword)) => Some(keyword.clone()),
            _ => None,
        }
    } else {
        None
    };

    SuiteDBLegacy {
        id: None,
        name: suite.name.clone(),
        source: suite.source_file.clone(),
        status: suite.status.status.clone(),
        start_time: map_timestamp(&suite.status.start_time).unwrap(),
        end_time: map_timestamp(&suite.status.end_time).unwrap(),
        identifier: suite.id.clone(),
        doc: suite.doc.clone(),
        setup_keyword,
        suites: suite
            .children
            .iter()
            .filter_map(|child| match child {
                parser::SuiteChildren::Suite(suite) => Some(map_suite(suite)),
                _ => None,
            })
            .collect(),
        tests: suite
            .children
            .iter()
            .filter_map(|child| match child {
                parser::SuiteChildren::Test(test) => Some(map_test(test)),
                _ => None,
            })
            .collect(),
        teardown_keyword,
    }
}

fn map_test(test: &parser::Test) -> TestDBLegacy {
    TestDBLegacy {
        id: None,
        name: test.name.clone(),
        line: test.line.parse::<i32>().unwrap(),
        identifier: test.id.clone(),
        tags: test.tags.clone(),
        status: test.status.status.clone(),
        start_time: map_timestamp(&test.status.start_time).unwrap(),
        end_time: map_timestamp(&test.status.end_time).unwrap(),
        doc: test.doc.clone(),
        timeout: test.timeout.clone(),
        keywords: test.keywords.clone(),
    }
}

fn map_statistics(statistics: &parser::Statistics) -> Vec<StatDBLegacy> {
    let mut stats = Vec::new();
    stats.push(map_statistic(&statistics.total.stats, StatTypeDB::Total));
    stats.extend(
        statistics
            .tags
            .stats
            .iter()
            .map(|start| map_statistic(start, StatTypeDB::Tag)),
    );
    stats.extend(
        statistics
            .suites
            .stats
            .iter()
            .map(|start| map_statistic(start, StatTypeDB::Suite)),
    );
    stats
}

fn map_statistic(statistic: &parser::StatisticsTag, stat_type: StatTypeDB) -> StatDBLegacy {
    StatDBLegacy {
        id: None,
        stat_type,
        pass_count: statistic.pass as i32,
        fail_count: statistic.fail as i32,
        skip_count: statistic.skip as i32,
        text: statistic.text.clone(),
        identifier: statistic.id.clone(),
        name: statistic.name.clone(),
    }
}

fn map_errors(errors: &parser::Errors) -> Vec<ErrorDBLegacy> {
    errors
        .messages
        .iter()
        .map(|message| map_error(message))
        .collect()
}

fn map_error(error: &parser::Message) -> ErrorDBLegacy {
    ErrorDBLegacy {
        id: None,
        timestamp: map_timestamp(&error.timestamp).unwrap(),
        level: error.level.clone(),
        content: error.value.clone(),
    }
}

fn map_timestamp(timestamp: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(&timestamp, "%Y%m%d %H:%M:%S%.3f")
}
