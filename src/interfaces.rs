use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronTimeUnit {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleValue {
    String(String),
    Number(i32),
}

impl fmt::Display for ScheduleValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScheduleValue::String(s) => write!(f, "{}", s),
            ScheduleValue::Number(n) => write!(f, "{}", n),
        }
    }
}
