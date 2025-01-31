INSERT INTO test_runs (rpa, generator, generated_date, schema_version)
VALUES ($1, $2, $3, $4)
RETURNING id;