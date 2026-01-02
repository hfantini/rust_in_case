use time::OffsetDateTime;
use time::macros::format_description;

pub fn now_formatted() -> String {
    let format = 
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    let current_date_time = 
        OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());

    current_date_time.format(&format).unwrap_or("????-??-?? ??:??:??".to_string())
}