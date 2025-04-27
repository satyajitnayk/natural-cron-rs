use regex::Regex;

pub struct CronValidators;

impl CronValidators {
    /// Validates if the minute is between 0 and 59
    pub fn validate_minute(minute: i32) -> Result<(), String> {
        if minute < 0 || minute > 59 {
            return Err(format!(
                "Invalid minute: {}. Minute should be between 0 and 59.",
                minute
            ));
        } else {
            Ok(())
        }
    }

    /// Validates if the hour is between 0 and 23
    pub fn validate_hour(hour: i32) -> Result<(), String> {
        if hour < 0 || hour > 23 {
            return Err(format!(
                "Invalid hour: {}. Hour should be between 0 and 23.",
                hour
            ));
        } else {
            Ok(())
        }
    }

    /// Validates if the day of month is between 1 and 31
    pub fn validate_day_of_month(day: i32) -> Result<(), String> {
        if day < 1 || day > 31 {
            return Err(format!(
                "Invalid day of month: {}. Day should be between 1 and 31.",
                day
            ));
        } else {
            Ok(())
        }
    }

    /// Validates if the month is between 1 and 12
    pub fn validate_month(month: i32) -> Result<(), String> {
        if month < 1 || month > 12 {
            return Err(format!(
                "Invalid month: {}. Month should be between 1 and 12.",
                month
            ));
        } else {
            Ok(())
        }
    }

    /// Validates if the day of week is between 0 and 6
    pub fn validate_day_of_week(day: i32) -> Result<(), String> {
        if day < 0 || day > 6 {
            return Err(format!(
                "Invalid day of week: {}. Day should be between 0 (Sunday) and 6 (Saturday).",
                day
            ));
        } else {
            Ok(())
        }
    }

    /// Validates if a time string matches HH:MM format
    pub fn validate_time(time: &str) -> Result<(), String> {
        let re = Regex::new(r"^([01]?[0-9]|2[0-3]):([0-5]?[0-9])$").unwrap();
        if re.is_match(time) {
            Ok(())
        } else {
            return Err(format!("Invalid time format for 'at': {}", time));
        }
    }

    /// Validates if the time unit is valid
    pub fn validate_time_unit(unit: &str) -> Result<(), String> {
        let valid_units = ["minute", "hour", "day", "month", "week"];
        if valid_units.contains(&unit) {
            Ok(())
        } else {
            return Err(format!("Invalid time unit for cron: {}", unit));
        }
    }
}
