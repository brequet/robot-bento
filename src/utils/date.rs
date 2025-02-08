use chrono::{Duration, NaiveDateTime};
use sqlx::postgres::types::PgInterval;

pub fn format_datetime(date: NaiveDateTime) -> String {
    date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn pg_interval_to_duration(pg_interval: PgInterval) -> Duration {
    let months = pg_interval.months as i64;
    let days = pg_interval.days as i64;
    let microseconds = pg_interval.microseconds;
    Duration::days(months * 30 + days) + Duration::microseconds(microseconds)
}

pub fn duration_to_string(duration: Duration) -> String {
    let seconds = duration.num_seconds();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
