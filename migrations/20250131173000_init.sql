CREATE TABLE test_runs (
    id SERIAL PRIMARY KEY,
    rpa BOOLEAN NOT NULL,
    generator TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    generated_date TIMESTAMP NOT NULL,
    imported_date TIMESTAMP DEFAULT NOW()
);