CREATE TABLE test_runs (
    id SERIAL PRIMARY KEY,
    rpa BOOLEAN NOT NULL,
    generator TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    generated_date TIMESTAMP NOT NULL,
    imported_date TIMESTAMP DEFAULT NOW()
);
CREATE TABLE suites (
    id SERIAL PRIMARY KEY,
    test_run_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    source TEXT NOT NULL,
    status TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    identifier TEXT NOT NULL,
    parent_suite_id INTEGER,
    doc TEXT
);
CREATE TYPE stat_type AS ENUM ('total', 'tag', 'suite');
CREATE TABLE test_run_statistics (
    id SERIAL PRIMARY KEY,
    test_run_id INTEGER NOT NULL,
    -- foreign key to test_runs.id
    stat_type stat_type NOT NULL,
    pass_count INTEGER NOT NULL,
    fail_count INTEGER NOT NULL,
    skip_count INTEGER NOT NULL,
    text TEXT NOT NULL,
    identifier TEXT,
    name TEXT
);
CREATE TABLE test_run_errors (
    id SERIAL PRIMARY KEY,
    test_run_id INTEGER NOT NULL,
    -- foreign key to test_runs.id
    timestamp TIMESTAMP NOT NULL,
    level TEXT NOT NULL,
    content TEXT NOT NULL
);