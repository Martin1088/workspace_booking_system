use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, Utc};
use loco_rs::prelude::{IntoResponse, Response};
use crate::response_type::error::ErrorType;

pub struct TimeFunctions;
impl TimeFunctions {
    pub fn start_of_day() -> i32 {
        Utc::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap_or(Utc::now().date_naive().into())
            .and_utc()
            .timestamp() as i32
    }

    pub fn end_of_day() -> i32 {
        Utc::now()
            .date_naive()
            .and_hms_opt(23, 59, 59)
            .unwrap_or(Utc::now().date_naive().into())
            .and_utc()
            .timestamp() as i32
    }

    pub fn get_timestamp_hour(given_date: DateTime<FixedOffset>, hour: u32) -> i32 {
        given_date
            .date_naive()
            .and_hms_opt(hour, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp() as i32
    }

    pub fn start_of_year(day: DateTime<FixedOffset>) -> Option<i32> {
        let start_of_year = NaiveDate::from_yo_opt(day.year(), 1)?.and_hms_opt(0, 0, 0)?;
        Some(start_of_year.and_utc().timestamp() as i32)
    }

    pub fn end_of_year(day: DateTime<FixedOffset>) -> Option<i32> {
        let current_year = day.year();
        Some(
            if (current_year % 4 == 0 && current_year % 100 != 0) || (current_year % 400 == 0) {
                NaiveDate::from_yo_opt(day.year(), 366)?.and_hms_opt(24, 59, 59)?
            } else {
                NaiveDate::from_yo_opt(day.year(), 365)?.and_hms_opt(0, 0, 0)?
            }
            .and_utc()
            .timestamp() as i32,
        )
    }

    pub fn rfc3339_to_unixtimestamp(day: &str) -> Result<(i32, i32), Response> {
        let given_date =
            DateTime::parse_from_rfc3339(day).map_err(|_e| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
        let start_of_day = given_date.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let end_of_day = given_date.date_naive().and_hms_opt(23, 59, 59).unwrap();

        Ok((
            start_of_day.and_utc().timestamp() as i32,
            end_of_day.and_utc().timestamp() as i32,
        ))
    }

    pub fn participants_of_year(day: &str) -> Result<(i32, i32), ErrorType> {
        let given_date =
            DateTime::parse_from_rfc3339(day).map_err(|_e| ErrorType::GivenTimeIsNotRCF3339)?;
        Ok((
            self::TimeFunctions::start_of_year(given_date).ok_or(ErrorType::NoDateFound)?,
            self::TimeFunctions::end_of_year(given_date).ok_or(ErrorType::NoDateFound)?,
        ))
    }
    pub fn get_monday(day: &str) -> Result<DateTime<FixedOffset>, Response> {
        let given_date =
            DateTime::parse_from_rfc3339(day).map_err(|_e| ErrorType::GivenTimeIsNotRCF3339.into_response())?;
        let weekday = given_date.weekday();
        let days_to_subtract = weekday.num_days_from_monday();
        let result = given_date - chrono::Duration::days(days_to_subtract.into());
        Ok(result)
    }

    pub fn handle_vec_rfc3339(current: Option<Vec<String>>) -> Result<String, Response> {
        current
            .ok_or(ErrorType::GivenTimeIsNotRCF3339.into_response())?
            .first()
            .cloned()
            .ok_or(ErrorType::GivenTimeIsNotRCF3339.into_response())
    }
}
