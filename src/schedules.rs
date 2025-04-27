//! Predefined cron expression constants.
//!
//! These constants provide commonly used schedules for cron jobs,
//! such as every minute, every hour, specific times of the day, weekdays only, etc.
//!
//! This module is intended for use with the `natural-cron` crate.
//! Example usage:
//! ```rust
//! use natural_cron::schedules;
//!
//! fn main() {
//!   println!(
//!       "Every 1st day of month at mid night: {}",
//!       schedules::EVERY_1ST_DAY_OF_MONTH_AT_MIDNIGHT
//!   );
//!   println!("Every 2 hours: {}", schedules::EVERY_2_HOURS);
//! }
//!

pub const EVERY_SECOND: &str = "* * * * * *";
pub const EVERY_5_SECONDS: &str = "*/5 * * * * *";
pub const EVERY_10_SECONDS: &str = "*/10 * * * * *";
pub const EVERY_30_SECONDS: &str = "*/30 * * * * *";
pub const EVERY_MINUTE: &str = "*/1 * * * *";
pub const EVERY_5_MINUTES: &str = "0 */5 * * * *";
pub const EVERY_10_MINUTES: &str = "0 */10 * * * *";
pub const EVERY_30_MINUTES: &str = "0 */30 * * * *";
pub const EVERY_HOUR: &str = "0 0-23/1 * * *";
pub const EVERY_2_HOURS: &str = "0 0-23/2 * * *";
pub const EVERY_3_HOURS: &str = "0 0-23/3 * * *";
pub const EVERY_4_HOURS: &str = "0 0-23/4 * * *";
pub const EVERY_5_HOURS: &str = "0 0-23/5 * * *";
pub const EVERY_6_HOURS: &str = "0 0-23/6 * * *";
pub const EVERY_7_HOURS: &str = "0 0-23/7 * * *";
pub const EVERY_8_HOURS: &str = "0 0-23/8 * * *";
pub const EVERY_9_HOURS: &str = "0 0-23/9 * * *";
pub const EVERY_10_HOURS: &str = "0 0-23/10 * * *";
pub const EVERY_11_HOURS: &str = "0 0-23/11 * * *";
pub const EVERY_12_HOURS: &str = "0 0-23/12 * * *";
pub const EVERY_DAY_AT_1AM: &str = "0 01 * * *";
pub const EVERY_DAY_AT_2AM: &str = "0 02 * * *";
pub const EVERY_DAY_AT_3AM: &str = "0 03 * * *";
pub const EVERY_DAY_AT_4AM: &str = "0 04 * * *";
pub const EVERY_DAY_AT_5AM: &str = "0 05 * * *";
pub const EVERY_DAY_AT_6AM: &str = "0 06 * * *";
pub const EVERY_DAY_AT_7AM: &str = "0 07 * * *";
pub const EVERY_DAY_AT_8AM: &str = "0 08 * * *";
pub const EVERY_DAY_AT_9AM: &str = "0 09 * * *";
pub const EVERY_DAY_AT_10AM: &str = "0 10 * * *";
pub const EVERY_DAY_AT_11AM: &str = "0 11 * * *";
pub const EVERY_DAY_AT_NOON: &str = "0 12 * * *";
pub const EVERY_DAY_AT_1PM: &str = "0 13 * * *";
pub const EVERY_DAY_AT_2PM: &str = "0 14 * * *";
pub const EVERY_DAY_AT_3PM: &str = "0 15 * * *";
pub const EVERY_DAY_AT_4PM: &str = "0 16 * * *";
pub const EVERY_DAY_AT_5PM: &str = "0 17 * * *";
pub const EVERY_DAY_AT_6PM: &str = "0 18 * * *";
pub const EVERY_DAY_AT_7PM: &str = "0 19 * * *";
pub const EVERY_DAY_AT_8PM: &str = "0 20 * * *";
pub const EVERY_DAY_AT_9PM: &str = "0 21 * * *";
pub const EVERY_DAY_AT_10PM: &str = "0 22 * * *";
pub const EVERY_DAY_AT_11PM: &str = "0 23 * * *";
pub const EVERY_DAY_AT_MIDNIGHT: &str = "0 0 * * *";
pub const EVERY_WEEK: &str = "0 0 * * 0";
pub const EVERY_WEEKDAY: &str = "0 0 * * 1-5";
pub const EVERY_WEEKEND: &str = "0 0 * * 6,0";
pub const EVERY_1ST_DAY_OF_MONTH_AT_MIDNIGHT: &str = "0 0 1 * *";
pub const EVERY_1ST_DAY_OF_MONTH_AT_NOON: &str = "0 12 1 * *";
pub const EVERY_2ND_HOUR: &str = "0 */2 * * *";
pub const EVERY_2ND_HOUR_FROM_1AM_THROUGH_11PM: &str = "0 1-23/2 * * *";
pub const EVERY_2ND_MONTH: &str = "0 0 1 */2 *";
pub const EVERY_QUARTER: &str = "0 0 1 */3 *";
pub const EVERY_6_MONTHS: &str = "0 0 1 */6 *";
pub const EVERY_YEAR: &str = "0 0 1 1 *";
pub const EVERY_30_MINUTES_BETWEEN_9AM_AND_5PM: &str = "0 */30 9-17 * * *";
pub const EVERY_30_MINUTES_BETWEEN_9AM_AND_6PM: &str = "0 */30 9-18 * * *";
pub const EVERY_30_MINUTES_BETWEEN_10AM_AND_7PM: &str = "0 */30 10-19 * * *";
pub const MONDAY_TO_FRIDAY_AT_1AM: &str = "0 0 01 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_2AM: &str = "0 0 02 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_3AM: &str = "0 0 03 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_4AM: &str = "0 0 04 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_5AM: &str = "0 0 05 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_6AM: &str = "0 0 06 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_7AM: &str = "0 0 07 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_8AM: &str = "0 0 08 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_9AM: &str = "0 0 09 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_09_30AM: &str = "0 30 09 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_10AM: &str = "0 0 10 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_11AM: &str = "0 0 11 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_11_30AM: &str = "0 30 11 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_12PM: &str = "0 0 12 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_1PM: &str = "0 0 13 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_2PM: &str = "0 0 14 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_3PM: &str = "0 0 15 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_4PM: &str = "0 0 16 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_5PM: &str = "0 0 17 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_6PM: &str = "0 0 18 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_7PM: &str = "0 0 19 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_8PM: &str = "0 0 20 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_9PM: &str = "0 0 21 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_10PM: &str = "0 0 22 * * 1-5";
pub const MONDAY_TO_FRIDAY_AT_11PM: &str = "0 0 23 * * 1-5";
