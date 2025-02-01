use chrono::{NaiveDateTime, ParseResult};

use crate::models::robot::{ErrorDB, StatDB, StatTypeDB, TestRunDB};
use crate::services::parser;

pub fn map_test_run(test_run: &parser::TestRun) -> Result<TestRunDB, chrono::ParseError> {
    Ok(TestRunDB {
        id: None,
        rpa: test_run.rpa,
        generator: test_run.generator.clone(),
        generated_date: map_timestamp(&test_run.generated_date.clone())?,
        schema_version: test_run.schema_version.clone(),
        statistics: map_statistics(&test_run.statistics),
        errors: map_errors(&test_run.errors),
    })
}

fn map_statistics(statistics: &parser::Statistics) -> Vec<StatDB> {
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

fn map_statistic(statistic: &parser::StatisticsTag, stat_type: StatTypeDB) -> StatDB {
    StatDB {
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

fn map_errors(errors: &parser::Errors) -> Vec<ErrorDB> {
    errors
        .messages
        .iter()
        .map(|message| map_error(message))
        .collect()
}

fn map_error(error: &parser::Message) -> ErrorDB {
    ErrorDB {
        id: None,
        timestamp: map_timestamp(&error.timestamp).unwrap(),
        level: error.level.clone(),
        content: error.value.clone(),
    }
}

fn map_timestamp(timestamp: &str) -> ParseResult<NaiveDateTime> {
    NaiveDateTime::parse_from_str(&timestamp, "%Y%m%d %H:%M:%S%.3f")
}
