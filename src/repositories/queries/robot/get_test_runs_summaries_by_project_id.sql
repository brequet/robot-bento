SELECT tr.project_id AS project_id,
    0 AS test_run_count,
    tr.id AS test_run_id,
    tr.application_version AS application_version,
    tr.generated_date AS test_run_date,
    timing.elapsed_time AS elapsed_time,
    stats.pass_count AS passed_tests,
    stats.fail_count AS failed_tests,
    stats.skip_count AS skipped_tests,
    errors.error_count AS error_count
FROM test_runs tr
    JOIN test_run_statistics stats ON stats.test_run_id = tr.id
    AND stats.stat_type = 'total'
    LEFT JOIN (
        SELECT test_run_id,
            COUNT(*)::INTEGER AS error_count
        FROM test_run_errors
        GROUP BY test_run_id
    ) errors ON errors.test_run_id = tr.id
    left join (
        SELECT test_run_id,
            s.end_time - s.start_time AS elapsed_time
        FROM suites s
        WHERE s.parent_suite_id IS NULL
    ) timing on timing.test_run_id = tr.id
WHERE tr.project_id = $1
ORDER BY tr.generated_date DESC;