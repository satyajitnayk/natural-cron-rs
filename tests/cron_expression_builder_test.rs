#[cfg(test)]
mod tests {
    use natural_cron::CronExpressionBuilder;

    fn new_schedule() -> CronExpressionBuilder {
        CronExpressionBuilder::new()
    }

    mod at_minutes {
        use std::vec;

        use super::*;

        #[test]
        fn sets_valid_minute() {
            let result = new_schedule()
                .at_minutes(vec![30])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("30 * * * *".to_string()));
        }

        #[test]
        fn accepts_zero_as_first_minute() {
            let result = new_schedule()
                .at_minutes(vec![0])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 * * * *".to_string()));
        }

        #[test]
        fn accepts_59_as_last_minute() {
            let result = new_schedule()
                .at_minutes(vec![59])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("59 * * * *".to_string()));
        }

        #[test]
        fn throws_when_minute_less_than_zero() {
            let result = new_schedule().at_minutes(vec![-1]);
            match result {
                Err(e) => {
                    assert_eq!(
                        e,
                        "Invalid minute: -1. Minute should be between 0 and 59.".to_string()
                    );
                }
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn throws_when_minute_greater_than_59() {
            let result = new_schedule().at_minutes(vec![60]);
            match result {
                Err(e) => {
                    assert_eq!(
                        e,
                        "Invalid minute: 60. Minute should be between 0 and 59.".to_string()
                    )
                }
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod at_hours {
        use super::*;

        #[test]
        fn sets_valid_hour() {
            let result = new_schedule()
                .at_hours(vec![12])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 12 * * *".to_string()));
        }

        #[test]
        fn accepts_zero_as_first_hour() {
            let result = new_schedule()
                .at_hours(vec![0])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 * * *".to_string()));
        }

        #[test]
        fn accepts_23_as_last_hour() {
            let result = new_schedule()
                .at_hours(vec![23])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 23 * * *".to_string()));
        }

        #[test]
        fn throws_when_hour_less_than_zero() {
            let result = new_schedule().at_hours(vec![-1]);
            match result {
                Err(e) => {
                    assert_eq!(
                        e,
                        "Invalid hour: -1. Hour should be between 0 and 23.".to_string()
                    );
                }
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn throws_when_hour_greater_than_23() {
            let result = new_schedule().at_hours(vec![24]);
            match result {
                Err(e) => {
                    assert_eq!(
                        e,
                        "Invalid hour: 24. Hour should be between 0 and 23.".to_string()
                    )
                }
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod every {

        use super::*;

        #[test]
        fn every_minute() {
            let result = new_schedule().every("minute").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * * * *".to_string()));
        }

        #[test]
        fn every_hour() {
            let result = new_schedule().every("hour").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 * * * *".to_string()));
        }

        #[test]
        fn every_day() {
            let result = new_schedule().every("day").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 * * *".to_string()));
        }

        #[test]
        fn every_month() {
            let result = new_schedule().every("month").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 1 * *".to_string()));
        }

        #[test]
        fn every_week() {
            let result = new_schedule().every("week").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 * * 0".to_string()));
        }

        #[test]
        fn throws_for_invalid_time_unit() {
            let result = new_schedule().every("decade");
            match result {
                Err(e) => assert_eq!(e, "Invalid time unit for cron: decade"),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod at_time {
        use super::*;

        #[test]
        fn sets_valid_time() {
            let result = new_schedule()
                .at_time("14:30")
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("30 14 * * *".to_string()));
        }

        #[test]
        fn sets_midnight() {
            let result = new_schedule()
                .at_time("00:00")
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 * * *".to_string()));
        }

        #[test]
        fn sets_noon() {
            let result = new_schedule()
                .at_time("12:00")
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 12 * * *".to_string()));
        }

        #[test]
        fn throws_for_invalid_hour() {
            let result = new_schedule().at_time("25:00");
            match result {
                Err(e) => assert_eq!(e, "Invalid time format for 'at': 25:00"),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn throws_for_invalid_minute() {
            let result = new_schedule().at_time("23:60");
            match result {
                Err(e) => assert_eq!(e, "Invalid time format for 'at': 23:60"),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn handles_no_leading_zero() {
            let result = new_schedule().at_time("7:5").and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("5 7 * * *".to_string()));
        }

        #[test]
        fn throws_for_invalid_format_extra_seconds() {
            let result = new_schedule().at_time("14:30:10");
            match result {
                Err(e) => assert_eq!(e, "Invalid time format for 'at': 14:30:10"),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn throws_for_impossible_time() {
            let result = new_schedule().at_time("25:61");
            match result {
                Err(e) => assert_eq!(e, "Invalid time format for 'at': 25:61"),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn overwrites_existing_time() {
            let result = new_schedule()
                .at_time("10:10")
                .and_then(|s| s.at_time("14:30"))
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("30 14 * * *".to_string()));
        }
    }

    mod on_week_days {
        use super::*;

        #[test]
        fn sets_single_day() {
            let result = new_schedule()
                .on_week_days(vec![3])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * * * 3".to_string()));
        }

        #[test]
        fn sets_multiple_days() {
            let result = new_schedule()
                .on_week_days(vec![1, 5])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * * * 1,5".to_string()));
        }

        #[test]
        fn rejects_invalid_day_high() {
            let result = new_schedule().on_week_days(vec![1, 8]);
            match result {
                Err(e) => assert_eq!(
                    e,
                    "Invalid day of week: 8. Day should be between 0 (Sunday) and 6 (Saturday)."
                ),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn rejects_inegative_day() {
            let result = new_schedule().on_week_days(vec![1, -3]);
            match result {
                Err(e) => assert_eq!(
                    e,
                    "Invalid day of week: -3. Day should be between 0 (Sunday) and 6 (Saturday)."
                ),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod on_days_of_month {
        use super::*;

        #[test]
        fn sets_single_day_of_month() {
            let result = new_schedule()
                .on_days_of_month(vec![15])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * 15 * *".to_string()));
        }

        #[test]
        fn sets_multipl_days_of_month() {
            let result = new_schedule()
                .on_days_of_month(vec![1, 10, 20])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * 1,10,20 * *".to_string()));
        }

        #[test]
        fn rejects_invalid_day_high() {
            let result = new_schedule().on_days_of_month(vec![32]);
            match result {
                Err(e) => assert_eq!(
                    e,
                    "Invalid day of month: 32. Day should be between 1 and 31."
                ),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn rejects_zero_day() {
            let result = new_schedule().on_days_of_month(vec![0]);
            match result {
                Err(e) => assert_eq!(
                    e,
                    "Invalid day of month: 0. Day should be between 1 and 31."
                ),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod during_months {
        use super::*;

        #[test]
        fn sets_single_month() {
            let result = new_schedule()
                .during_months(vec![6])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * * 6 *".to_string()));
        }

        #[test]
        fn sets_multiple_months() {
            let result = new_schedule()
                .during_months(vec![2, 4, 6])
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("* * * 2,4,6 *".to_string()));
        }

        #[test]
        fn rejects_invalid_month_zero() {
            let result = new_schedule().during_months(vec![0]);
            match result {
                Err(e) => assert_eq!(e, "Invalid month: 0. Month should be between 1 and 12."),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }

        #[test]
        fn rejects_invalid_month_high() {
            let result = new_schedule().during_months(vec![13]);
            match result {
                Err(e) => assert_eq!(e, "Invalid month: 13. Month should be between 1 and 12."),
                Ok(_) => panic!("Expected error but got Ok"),
            }
        }
    }

    mod every_x {
        use super::*;
        use natural_cron::interfaces::CronTimeUnit;

        #[test]
        fn every_15_minutes() {
            let result = new_schedule()
                .every_x(15, CronTimeUnit::Minute)
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("*/15 * * * *".to_string()));
        }

        #[test]
        fn every_3_hours() {
            let result = new_schedule()
                .every_x(3, CronTimeUnit::Hour)
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 */3 * * *".to_string()));
        }

        #[test]
        fn every_5_days_of_month() {
            let result = new_schedule()
                .every_x(5, CronTimeUnit::DayOfMonth)
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 */5 * *".to_string()));
        }

        #[test]
        fn every_2_months() {
            let result = new_schedule()
                .every_x(2, CronTimeUnit::Month)
                .and_then(|s| Ok(s.compile()));
            assert_eq!(result, Ok("0 0 1 */2 *".to_string()));
        }
    }
}
