SELECT s.id,
    s.name,
    s.source,
    s.status,
    s.start_time,
    s.end_time,
    s.identifier,
    s.doc
FROM suites s
WHERE s.test_run_id = $1
    AND s.parent_suite_id IS NULL
ORDER BY s.start_time ASC;