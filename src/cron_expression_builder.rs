use std::collections::HashMap;

use crate::cron_utils::CronUtils;
use crate::cron_validator::CronValidators;
use crate::interfaces::{CronTimeUnit, ScheduleValue};

#[derive(Debug, Default, Clone)]
pub struct CronExpressionBuilder {
    schedule: HashMap<String, ScheduleValue>,
}

impl CronExpressionBuilder {
    /// Creates a new instance of the builder
    pub fn new() -> Self {
        Self {
            schedule: HashMap::new(),
        }
    }

    /// Ensure default values for all cron parts
    fn ensure_default_values(&mut self) {
        CronUtils::set_default(
            &mut self.schedule,
            "minute",
            ScheduleValue::String("*".to_string()),
        );
        CronUtils::set_default(
            &mut self.schedule,
            "hour",
            ScheduleValue::String("*".to_string()),
        );
        CronUtils::set_default(
            &mut self.schedule,
            "dayOfMonth",
            ScheduleValue::String("*".to_string()),
        );
        CronUtils::set_default(
            &mut self.schedule,
            "month",
            ScheduleValue::String("*".to_string()),
        );
        CronUtils::set_default(
            &mut self.schedule,
            "dayOfWeek",
            ScheduleValue::String("*".to_string()),
        );
    }

    /// At specific minutes
    pub fn at_minutes(mut self, minutes: Vec<i32>) -> Result<Self, String> {
        for minute in &minutes {
            // ? — it propagates the error automatically if any validation fails.
            CronValidators::validate_minute(*minute)?;
        }
        let formatted = CronUtils::format_cron_part(&minutes);
        self.schedule
            .insert("minute".to_string(), ScheduleValue::String(formatted));
        Ok(self)
    }

    /// At specific hours
    pub fn at_hours(mut self, hours: Vec<i32>) -> Result<Self, String> {
        for hour in &hours {
            CronValidators::validate_hour(*hour)?;
        }
        let formatted = CronUtils::format_cron_part(&hours);
        self.schedule
            .insert("hour".to_string(), ScheduleValue::String(formatted));

        if !self.schedule.contains_key("minute") {
            self.schedule
                .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
        }
        Ok(self)
    }

    pub fn at_time(mut self, time: &str) -> Result<Self, String> {
        CronValidators::validate_time(time)?;

        let parts: Vec<&str> = time.split(":").collect();
        let hour = parts[0].parse::<i32>().unwrap();
        let minute = parts[1].parse::<i32>().unwrap();

        self.schedule.insert(
            "minute".to_string(),
            ScheduleValue::String(minute.to_string()),
        );
        self.schedule
            .insert("hour".to_string(), ScheduleValue::String(hour.to_string()));
        Ok(self)
    }

    /// Every unit (minute, hour, day, etc.)
    pub fn every(mut self, unit: &str) -> Result<Self, String> {
        CronValidators::validate_time_unit(unit)?;

        match unit {
            "minute" => {
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("*".to_string()));
            }
            "hour" => {
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("*".to_string()));
            }
            "day" => {
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfMonth".to_string(),
                    ScheduleValue::String("*".to_string()),
                );
            }
            "month" => {
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfMonth".to_string(),
                    ScheduleValue::String("1".to_string()),
                );
                self.schedule
                    .insert("month".to_string(), ScheduleValue::String("*".to_string()));
            }
            "week" => {
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfWeek".to_string(),
                    ScheduleValue::String("0".to_string()),
                );
                self.schedule.insert(
                    "dayOfMonth".to_string(),
                    ScheduleValue::String("*".to_string()),
                );
                self.schedule
                    .insert("month".to_string(), ScheduleValue::String("*".to_string()));
            }
            _ => {}
        }
        Ok(self)
    }

    /// Every X unit (every X minutes, hours, etc.)
    pub fn every_x(mut self, interval: i32, unit: CronTimeUnit) -> Result<Self, String> {
        match unit {
            CronTimeUnit::Minute => {
                CronValidators::validate_minute(interval)?;
                self.schedule.insert(
                    "minute".to_string(),
                    ScheduleValue::String(format!("*/{}", interval)),
                );
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("*".to_string()));
            }
            CronTimeUnit::Hour => {
                CronValidators::validate_hour(interval)?;
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "hour".to_string(),
                    ScheduleValue::String(format!("*/{}", interval)),
                );
            }
            CronTimeUnit::DayOfMonth => {
                CronValidators::validate_day_of_month(interval)?;
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfMonth".to_string(),
                    ScheduleValue::String(format!("*/{}", interval)),
                );
            }
            CronTimeUnit::Month => {
                CronValidators::validate_month(interval)?;
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfMonth".to_string(),
                    ScheduleValue::String("1".to_string()),
                );
                self.schedule.insert(
                    "month".to_string(),
                    ScheduleValue::String(format!("*/{}", interval)),
                );
            }
            CronTimeUnit::DayOfWeek => {
                CronValidators::validate_day_of_week(interval)?;
                self.schedule
                    .insert("minute".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule
                    .insert("hour".to_string(), ScheduleValue::String("0".to_string()));
                self.schedule.insert(
                    "dayOfWeek".to_string(),
                    ScheduleValue::String(format!("*/{}", interval)),
                );
            }
        }
        Ok(self)
    }

    /// On specific weekdays
    pub fn on_week_days(mut self, days: Vec<i32>) -> Result<Self, String> {
        for day in &days {
            CronValidators::validate_day_of_week(*day)?;
        }
        let formatted = CronUtils::format_cron_part(&days);
        self.schedule
            .insert("dayOfWeek".to_string(), ScheduleValue::String(formatted));
        Ok(self)
    }

    /// On specific days of the month
    pub fn on_days_of_month(mut self, days: Vec<i32>) -> Result<Self, String> {
        for day in &days {
            CronValidators::validate_day_of_month(*day)?;
        }
        let formatted = CronUtils::format_cron_part(&days);
        self.schedule
            .insert("dayOfMonth".to_string(), ScheduleValue::String(formatted));
        Ok(self)
    }

    /// During specific months
    pub fn during_months(mut self, months: Vec<i32>) -> Result<Self, String> {
        for month in &months {
            CronValidators::validate_month(*month)?;
        }
        let formatted = CronUtils::format_cron_part(&months);
        self.schedule
            .insert("month".to_string(), ScheduleValue::String(formatted));
        Ok(self)
    }

    /// Compile the schedule into a final cron expression
    pub fn compile(mut self) -> String {
        self.ensure_default_values();

        let minute = self.schedule.get("minute").unwrap().to_string();
        let hour = self.schedule.get("hour").unwrap().to_string();
        let day_of_month = self.schedule.get("dayOfMonth").unwrap().to_string();
        let month = self.schedule.get("month").unwrap().to_string();
        let day_of_week = self.schedule.get("dayOfWeek").unwrap().to_string();

        format!(
            "{} {} {} {} {}",
            minute, hour, day_of_month, month, day_of_week
        )
    }
}
