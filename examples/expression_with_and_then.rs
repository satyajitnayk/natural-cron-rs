use natural_cron::CronExpressionBuilder;

fn main() {
    let cron = CronExpressionBuilder::new()
        .at_time("08:00")
        .and_then(|s| s.on_week_days(vec![1, 2, 3, 4, 5]))
        .and_then(|s| Ok(s.compile()));

    match cron {
        Ok(cron_string) => println!("Generated cron: {}", cron_string),
        Err(e) => eprintln!("Failed to generate cron: {}", e),
    }
}
