use std::collections::HashMap;

use crate::interfaces::ScheduleValue;

pub struct CronUtils;

impl CronUtils {
    /// Checks if the numbers are contiguous
    pub fn is_contiguous(numbers: &[i32]) -> bool {
        if numbers.is_empty() {
            return true;
        }
        let mut sorted_numbers = numbers.to_vec();
        sorted_numbers.sort_unstable();

        for i in 1..sorted_numbers.len() {
            if sorted_numbers[i] != sorted_numbers[i - 1] + 1 {
                return false;
            }
        }
        true
    }

    /// Formats a list of numbers into a cron-compatible string
    pub fn format_cron_part(values: &[i32]) -> String {
        let mut unique_values = values.to_vec();
        unique_values.sort_unstable();
        unique_values.dedup();

        let contiguous = CronUtils::is_contiguous(&unique_values);

        if contiguous && unique_values.len() > 2 {
            format!(
                "{}-{}",
                unique_values[0],
                unique_values[unique_values.len() - 1]
            )
        } else {
            unique_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        }
    }

    /// Sets a default value for a field in the schedule if not already set
    pub fn set_default(
        sehedule: &mut HashMap<String, ScheduleValue>,
        field: &str,
        default_value: ScheduleValue,
    ) {
        sehedule.entry(field.to_string()).or_insert(default_value);
    }
}
