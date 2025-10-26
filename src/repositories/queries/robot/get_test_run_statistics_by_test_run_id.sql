SELECT stats.id,
    stats.stat_type as "stat_type: StatTypeDB",
    stats.pass_count,
    stats.fail_count,
    stats.skip_count,
    stats.identifier,
    stats.name,
    stats.text
FROM test_run_statistics stats
WHERE stats.test_run_id = $1;