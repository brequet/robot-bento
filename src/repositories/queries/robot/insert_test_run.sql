INSERT INTO test_runs (
        project_id,
        rpa,
        generator,
        generated_date,
        schema_version,
        application_version,
        sha1
    )
VALUES ($1, $2, $3, $4, $5, $6, $7)
RETURNING id;