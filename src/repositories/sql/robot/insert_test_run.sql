INSERT INTO test_runs (
        rpa,
        generator,
        generated_date,
        schema_version,
        application_name,
        application_version,
        sha1
    )
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id;