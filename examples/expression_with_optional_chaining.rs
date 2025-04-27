use natural_cron::CronExpressionBuilder;

fn build_cron() -> Result<String, String> {
    let builder = CronExpressionBuilder::new()
        .at_time("08:00")?
        .on_week_days(vec![1, 2, 3, 4, 5])?;
    Ok(builder.compile())
}

fn main() {
    match build_cron() {
        Ok(cron_string) => println!("Generated cron: {}", cron_string),
        Err(e) => eprintln!("Failed to generate cron: {}", e),
    }
}
