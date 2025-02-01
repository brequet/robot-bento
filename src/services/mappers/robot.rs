use chrono::NaiveDateTime;

use crate::models::robot::{StatDB, StatTypeDB, TestRunDB};
use crate::services::parser;

pub fn map_test_run(test_run: &parser::TestRun) -> Result<TestRunDB, chrono::ParseError> {
    Ok(TestRunDB {
        id: None,
        rpa: test_run.rpa,
        generator: test_run.generator.clone(),
        generated_date: NaiveDateTime::parse_from_str(
            &test_run.generated_date.clone(),
            "%Y%m%d %H:%M:%S%.3f",
        )?,
        schema_version: test_run.schema_version.clone(),
        statistics: map_statistics(&test_run.statistics),
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
