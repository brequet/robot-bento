WITH total_count AS (
    SELECT project_id,
        COUNT(*)::INTEGER as total_rows
    FROM test_runs tr
    WHERE tr.project_id IN (
            SELECT unnest($1::integer [])
        )
    GROUP BY project_id
)
SELECT DISTINCT ON (tr.project_id) tr.project_id,
    total_count.total_rows as test_run_count,
    tr.id as test_run_id,
    tr.application_version as application_version,
    tr.generated_date as test_run_date,
    stats.pass_count as passed_tests,
    stats.fail_count as failed_tests,
    stats.skip_count as skipped_tests,
    errors.error_count as error_count,
    timing.elapsed_time as elapsed_time
FROM test_runs tr
    JOIN test_run_statistics stats ON stats.test_run_id = tr.id
    JOIN total_count ON total_count.project_id = tr.project_id
    LEFT JOIN (
        SELECT test_run_id,
            COUNT(*)::INTEGER as error_count
        FROM test_run_errors
        GROUP BY test_run_id
    ) errors ON errors.test_run_id = tr.id
    LEFT JOIN (
        SELECT test_run_id,
            s.end_time - s.start_time AS elapsed_time
        FROM suites s
        WHERE s.parent_suite_id IS NULL
    ) timing ON timing.test_run_id = tr.id
WHERE tr.project_id IN (
        SELECT unnest($1::integer [])
    )
    AND stats.stat_type = 'total'
ORDER BY tr.project_id,
    tr.generated_date DESC;