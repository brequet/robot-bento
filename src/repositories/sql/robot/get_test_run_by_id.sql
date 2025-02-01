SELECT tr.id,
    tr.rpa,
    tr.generator,
    tr.generated_date,
    tr.schema_version
FROM test_runs tr
WHERE tr.id = $1