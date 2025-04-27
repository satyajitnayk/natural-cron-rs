use natural_cron::schedules;

fn main() {
    println!(
        "Every 1st day of month at mid night: {}",
        schedules::EVERY_1ST_DAY_OF_MONTH_AT_MIDNIGHT
    );
    println!("Every 2 hours: {}", schedules::EVERY_2_HOURS);
}
