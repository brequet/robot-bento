WITH total_count AS (
    SELECT project_id, 
        application_version,
        COUNT(*)::INTEGER as total_rows
    FROM test_runs tr
    WHERE tr.project_id IN (
            SELECT unnest($1::integer[])
        )
    GROUP BY project_id, application_version
)
SELECT DISTINCT ON (tr.project_id, tr.application_version) tr.project_id,
    tr.application_version,
    total_count.total_rows as test_run_count,
    tr.generated_date as last_test_run_date,
    stats.pass_count as last_passed_tests,
    stats.fail_count as last_failed_tests,
    stats.skip_count as last_skipped_tests
FROM test_runs tr
    JOIN test_run_statistics stats ON stats.test_run_id = tr.id
    JOIN total_count ON total_count.project_id = tr.project_id and total_count.application_version = tr.application_version
WHERE tr.project_id IN (
        SELECT unnest($1::integer[])
    )
    AND stats.stat_type = 'total'
ORDER BY tr.project_id, 
	tr.application_version DESC,
    tr.generated_date DESC;