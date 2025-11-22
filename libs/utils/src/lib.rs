use chrono::{NaiveDateTime, TimeZone, Utc};

pub fn convert_timestamp(date_str: String) -> Result<i64, chrono::ParseError> {
    let naive_datetime = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M:%S")?;
    let datetime_utc = Utc.from_local_datetime(&naive_datetime).unwrap();
    let timestamp = datetime_utc.timestamp_millis();
    Ok(timestamp - ((8 * 3600 * 1000) as i64))
}

pub fn advanced_round(input: f64, precision: i32) -> f64 {
    let factor = 10_f64.powi(precision);
    (input * factor).round() / factor
}
