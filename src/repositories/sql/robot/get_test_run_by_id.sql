SELECT tr.id,
    tr.rpa,
    tr.generator,
    tr.generated_date,
    tr.schema_version,
    s.id as stat_id,
    s.stat_type as "stat_type: StatTypeDB",
    s.pass_count,
    s.fail_count,
    s.skip_count,
    s.identifier,
    s.name,
    s.text
FROM test_runs tr
    LEFT JOIN statistics s ON s.test_run_id = tr.id
WHERE tr.id = $1