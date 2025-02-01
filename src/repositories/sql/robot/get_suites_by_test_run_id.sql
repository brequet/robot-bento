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
ORDER BY s.start_time ASC;