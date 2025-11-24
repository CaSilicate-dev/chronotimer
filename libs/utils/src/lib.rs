use chrono::{Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use std::cmp::min;
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

#[derive(Debug)]
pub struct SplitedTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}
impl SplitedTime {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    pub fn from_string(datetime: String) -> Result<Self, chrono::ParseError> {
        let dt = NaiveDateTime::parse_from_str(datetime.as_str(), "%Y-%m-%d %H:%M:%S")?;
        Ok(Self {
            year: dt.year(),
            month: dt.month() as i32,
            day: dt.day() as i32,
            hour: dt.hour() as i32,
            minute: dt.minute() as i32,
            second: dt.second() as i32,
        })
    }

    pub fn to_string(self) -> String {
        format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day,
            self.hour, self.minute, self.second)
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
