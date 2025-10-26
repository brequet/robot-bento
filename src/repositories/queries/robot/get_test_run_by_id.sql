SELECT tr.id,
    tr.rpa,
    tr.generator,
    tr.generated_date,
    tr.schema_version,
    tr.application_version,
    tr.sha1,
    tr.imported_date
FROM test_runs tr
WHERE tr.id = $1;