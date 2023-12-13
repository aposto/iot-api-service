use chrono::{DateTime, Local};

pub fn datetime_string(dt: DateTime<Local>) -> String {
    return dt.format("%Y-%m-%dT%H:%M:%S").to_string()
}
