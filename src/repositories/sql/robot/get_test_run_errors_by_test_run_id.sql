SELECT errors.id,
    errors.timestamp,
    errors.level,
    errors.content
FROM test_run_errors errors
WHERE errors.test_run_id = $1;