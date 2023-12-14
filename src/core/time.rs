use chrono::{DateTime, Local};

pub fn datetime_string(dt: DateTime<Local>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn local_datetime_string() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}