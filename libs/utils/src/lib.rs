use chrono::{NaiveDateTime, TimeZone, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Invalid unit: '{unit}',")]
pub struct InvalidUnitError {
    pub unit: String,
}
impl InvalidUnitError {
    pub fn new(unit: &str) -> Self {
        Self {
            unit: unit.to_string(),
        }
    }
}
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

pub fn format_zeros(number: f64, decimals: i32) -> String {
    format!("{:.decimals$}", number, decimals = decimals as usize)
}

pub fn convert_time_unit(timems: f64, unit: &str) -> Result<f64, InvalidUnitError> {
    if unit == "ms" {
        Ok(timems)
    } else if unit == "s" {
        Ok(timems / (1_i64 * 1000) as f64)
    } else if unit == "m" {
        Ok(timems / (1_i64 * 1000 * 60) as f64)
    } else if unit == "h" {
        Ok(timems / (1_i64 * 1000 * 60 * 60) as f64)
    } else if unit == "d" {
        Ok(timems / (1_i64 * 1000 * 60 * 60 * 24) as f64)
    } else if unit == "w" {
        Ok(timems / (1_i64 * 1000 * 60 * 60 * 24 * 7) as f64)
    } else if unit == "mo" {
        Ok(timems / (1_i64 * 1000 * 60 * 60 * 24 * 30) as f64)
    } else if unit == "y" {
        Ok(timems / (1_i64 * 1000 * 60 * 60 * 24 * 365) as f64)
    } else {
        Err(InvalidUnitError::new(unit))
    }
}