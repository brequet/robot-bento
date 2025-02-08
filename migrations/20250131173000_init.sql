CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    create_date TIMESTAMP DEFAULT NOW() NOT NULL
);
CREATE TABLE test_runs (
    id SERIAL PRIMARY KEY,
    project_id INTEGER NOT NULL,
    rpa BOOLEAN NOT NULL,
    generator TEXT NOT NULL,
    schema_version TEXT NOT NULL,
    generated_date TIMESTAMP NOT NULL,
    application_version TEXT NOT NULL,
    sha1 TEXT UNIQUE NOT NULL,
    imported_date TIMESTAMP DEFAULT NOW() NOT NULL
);
CREATE TABLE suites (
    id SERIAL PRIMARY KEY,
    test_run_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    name TEXT NOT NULL,
    source TEXT NOT NULL,
    status TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    parent_suite_id INTEGER,
    doc TEXT
);
CREATE TABLE suite_keywords (
    suite_id INTEGER NOT NULL,
    type TEXT NOT NULL,
    value JSONB NOT NULL
);
CREATE TABLE tests (
    id SERIAL PRIMARY KEY,
    suite_id INTEGER NOT NULL,
    identifier TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    line INTEGER NOT NULL,
    doc TEXT,
    timeout TEXT
);
CREATE TABLE test_keywords (
    test_id INTEGER NOT NULL,
    value JSONB NOT NULL
);
CREATE TABLE test_tags (
    test_id INTEGER NOT NULL,
    value TEXT NOT NULL,
    PRIMARY KEY (test_id, value)
);
CREATE INDEX idx_tags_test_id ON test_tags (test_id);
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