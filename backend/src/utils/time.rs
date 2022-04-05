use chrono::{Datelike, TimeZone};

pub fn current_day_start_local() -> chrono::DateTime<chrono::Local> {
    day_start_from_datetime(chrono::Local::now())
}

pub fn day_start_from_datetime(
    dt: chrono::DateTime<chrono::Local>,
) -> chrono::DateTime<chrono::Local> {
    chrono::Local
        .ymd(dt.year(), dt.month(), dt.day())
        .and_hms(0, 0, 0)
}

pub struct DateTimeFromStrError(pub String);

pub fn date_time_from_str(
    time: &str,
) -> Result<chrono::DateTime<chrono::Local>, DateTimeFromStrError> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(time) {
        Ok(dt.into())
    } else if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(time) {
        Ok(dt.into())
    } else {
        Err(DateTimeFromStrError(time.to_string()))
    }
}
