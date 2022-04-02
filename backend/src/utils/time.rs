use chrono::{Datelike, TimeZone};

pub fn current_day_start_local() -> chrono::DateTime<chrono::Local> {
    let dt = chrono::Local::now();
    chrono::Local
        .ymd(dt.year(), dt.month(), dt.day())
        .and_hms(0, 0, 0)
}
