#[cfg(test)]
mod tests {
    use std::vec;

    use natural_cron::{CronExpressionBuilder, interfaces::CronTimeUnit};

    fn new_schedule() -> CronExpressionBuilder {
        CronExpressionBuilder::new()
    }

    #[test]
    fn run_job_at_5_30_pm_every_day() {
        let result = new_schedule()
            .at_time("17:30")
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("30 17 * * *".to_string()));
    }

    #[test]
    fn run_at_9_am_on_weekdays() {
        let result = new_schedule()
            .at_time("09:00")
            .and_then(|s| s.on_week_days(vec![1, 2, 3, 4, 5]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 9 * * 1-5".to_string()));
    }

    #[test]
    fn run_at_noon_on_1st_and_15th() {
        let result = new_schedule()
            .at_time("12:00")
            .and_then(|s| s.on_days_of_month(vec![1, 15]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 12 1,15 * *".to_string()));
    }

    #[test]
    fn run_at_midnight_in_january_and_july() {
        let result = new_schedule()
            .at_time("00:00")
            .and_then(|s| s.during_months(vec![1, 7]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 0 * 1,7 *".to_string()));
    }

    #[test]
    fn run_every_15_minutes() {
        let result = new_schedule()
            .every_x(15, CronTimeUnit::Minute)
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("*/15 * * * *".to_string()));
    }

    #[test]
    fn run_every_day_at_noon() {
        let result = new_schedule()
            .every("day")
            .and_then(|s| s.at_hours(vec![12]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 12 * * *".to_string()));
    }

    #[test]
    fn run_every_sunday_at_5_pm() {
        let result = new_schedule()
            .on_week_days(vec![0])
            .and_then(|s| s.at_hours(vec![17]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 17 * * 0".to_string()));
    }

    #[test]
    fn run_1st_day_of_every_month_at_1_am() {
        let result = new_schedule()
            .on_days_of_month(vec![1])
            .and_then(|s| s.every("month"))
            .and_then(|s| s.at_hours(vec![1]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 1 1 * *".to_string()));
    }

    #[test]
    fn run_every_weekday_at_8_30_am() {
        let result = new_schedule()
            .at_time("08:30")
            .and_then(|s| s.on_week_days(vec![1, 2, 3, 4, 5]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("30 8 * * 1-5".to_string()));
    }

    #[test]
    fn run_every_6_hours() {
        let result = new_schedule()
            .every_x(6, CronTimeUnit::Hour)
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 */6 * * *".to_string()));
    }

    #[test]
    fn run_every_quarter_at_midnight() {
        let result = new_schedule()
            .during_months(vec![1, 4, 7, 10])
            .and_then(|s| s.on_days_of_month(vec![1]))
            .and_then(|s| s.at_time("00:00"))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 0 1 1,4,7,10 *".to_string()));
    }

    #[test]
    fn run_every_saturday_and_sunday_at_10_15_am() {
        let result = new_schedule()
            .at_time("10:15")
            .and_then(|s| s.on_week_days(vec![6, 0]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("15 10 * * 0,6".to_string()));
    }

    #[test]
    fn run_at_9am_12pm_3pm_every_day() {
        let result = new_schedule()
            .every("day")
            .and_then(|s| s.at_hours(vec![9, 12, 15]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 9,12,15 * * *".to_string()));
    }

    #[test]
    fn run_at_7_2_10_on_tuesdays() {
        let result = new_schedule()
            .on_week_days(vec![2])
            .and_then(|s| s.at_hours(vec![7, 14, 22]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("0 7,14,22 * * 2".to_string()));
    }

    #[test]
    fn run_at_20_past_every_hour_on_5th_of_july() {
        let result = new_schedule()
            .on_days_of_month(vec![5])
            .and_then(|s| s.at_minutes(vec![20]))
            .and_then(|s| s.during_months(vec![7]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("20 * 5 7 *".to_string()));
    }

    #[test]
    fn run_every_5_minute_office_hours_using_every() {
        let result = new_schedule()
            .every("minute")
            .and_then(|s| s.at_minutes(vec![0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55]))
            .and_then(|s| s.at_hours(vec![9, 10, 11, 12, 13, 14, 15, 16]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(
            result,
            Ok("0,5,10,15,20,25,30,35,40,45,50,55 9-16 * * *".to_string())
        );
    }

    #[test]
    fn run_every_5_minute_office_hours_using_every_x() {
        let result = new_schedule()
            .every_x(5, CronTimeUnit::Minute)
            .and_then(|s| s.at_hours(vec![9, 10, 11, 12, 13, 14, 15, 16]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("*/5 9-16 * * *".to_string()));
    }

    #[test]
    fn run_quarter_past_and_quarter_to_every_hour() {
        let result = new_schedule()
            .every("hour")
            .and_then(|s| s.at_minutes(vec![15, 45]))
            .and_then(|s| Ok(s.compile()));
        assert_eq!(result, Ok("15,45 * * * *".to_string()));
    }
}
