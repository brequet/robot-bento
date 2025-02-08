use chrono::NaiveDateTime;

pub fn format_datetime(date: NaiveDateTime) -> String {
    date.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}
