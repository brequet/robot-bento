SELECT t.id,
    t.identifier,
    t.name,
    t.status,
    t.start_time,
    t.end_time,
    t.line,
    t.doc,
    t.timeout
FROM tests t
WHERE t.suite_id = $1
ORDER BY t.start_time ASC;