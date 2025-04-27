# Natural Cron ⏰ (Rust)

[![Crates.io](https://img.shields.io/crates/v/natural-cron.svg)](https://crates.io/crates/natural-cron)
[![Documentation](https://docs.rs/natural-cron-rs/badge.svg)](https://docs.rs/natural-cron)
[![License](https://img.shields.io/crates/l/natural-cron-rs.svg)](https://github.com/satyajitnayk/natural-cron-rs/blob/main/LICENSE)

---

**Natural Cron** is an easy-to-use Rust library for building and validating cron expressions with a clean, human-readable API.

---

## ✨ Features

- ✅ Chainable builder methods for constructing cron expressions.
- ✅ Validation for all cron components (minute, hour, day of month, month, day of week).
- ✅ Helpful error messages for invalid inputs.
- ✅ Strong typing using Rust enums and strict validation.
- ✅ No dependencies — lightweight and efficient.

---

## 📦 Installation

Add `natural-cron` to your `Cargo.toml`:

```toml
[dependencies]
natural-cron = "0.1"
```

Then in your code:

```rust
use natural_cron::CronExpressionBuilder;
use natural_cron::CronTimeUnit;
```

---

## 🚀 Usage

Use `CronExpressionBuilder` to create cron expressions easily:

```rust
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
```

or simle chaining

```rust
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
```

---
## 📋 Examples Table

| Description                                   | Code Example                                                              | Cron Expression |
| --------------------------------------------- | ------------------------------------------------------------------------- | --------------- |
| Run job at 5:30 PM every day                  | `CronExpressionBuilder::new().at_time("17:30").and_then(Ok(s.compile()));`                                  | `30 17 * * *`  |
| Run at 9 AM on weekdays                       | `CronExpressionBuilder::new().at_time("09:00").and_then(s.on_week_days(vec![1,2,3,4,5])).and_then(Ok(s.compile()));` | `0 9 * * 1-5`     |
| Run at noon on 1st and 15th                   | `CronExpressionBuilder::new().at_time("12:00").and_then(s.on_days_of_month(vec![1, 15])).and_then(Ok(s.compile()));` | `0 12 1,15 * *`   |
| Run at midnight during January and July       | `CronExpressionBuilder::new().at_time("00:00").and_then(s.during_months(vec![1, 7])).and_then(Ok(s.compile()));` | `0 0 * 1,7 *`     |
| Run every 15 minutes                          | `CronExpressionBuilder::new().every_x(15, CronTimeUnit::Minute).and_then(Ok(s.compile()));`                                  | `*/15 * * * *` |
| Run every day at noon                         | `CronExpressionBuilder::new().every("day").and_then(s.at_hours(vec![12])).and_then(Ok(s.compile()));`                  | `0 12 * * *`      |
| Run every Sunday at 5 PM                      | `CronExpressionBuilder::new().on_week_days(vec![0]).and_then(s.at_hours(vec![17])).and_then(Ok(s.compile()));` | `0 17 * * 0`      |
| Run 1st day of month at 1 AM                  | `CronExpressionBuilder::new().on_days_of_month(vec![1]).and_then(s.every("month")).and_then(s.at_hours(vec![1])).and_then(Ok(s.compile()));` | `0 1 1 * *`        |
| Run weekdays at 8:30 AM                       | `CronExpressionBuilder::new().at_time("08:30").and_then(s.on_week_days(vec![1,2,3,4,5])).and_then(Ok(s.compile()));` | `30 8 * * 1-5`    |
| Run every 6 hours                             | `CronExpressionBuilder::new().every_x(6, CronTimeUnit::Hour).and_then(Ok(s.compile()));`                                  | `0 */6 * * *`  |
| Run every quarter at midnight                 | `CronExpressionBuilder::new().during_months(vec![1,4,7,10]).and_then(s.on_days_of_month(vec![1])).and_then(s.at_time("00:00")).and_then(Ok(s.compile()));` | `0 0 1 1,4,7,10 *` |
| Run Sat and Sun at 10:15 AM                   | `CronExpressionBuilder::new().at_time("10:15").and_then(s.on_week_days(vec![6,0])).and_then(Ok(s.compile()));` | `15 10 * * 6,0`   |
| Run at 9, 12, 3 every day                     | `CronExpressionBuilder::new().every("day").and_then(s.at_hours(vec![9,12,15])).and_then(Ok(s.compile()));` | `0 9,12,15 * * *` |
| Run 7 AM, 2 PM, 10 PM on Tuesdays             | `CronExpressionBuilder::new().at_hours(vec![7,14,22]).and_then(s.on_week_days(vec![2])).and_then(Ok(s.compile()));` | `0 7,14,22 * * 2` |
| Run at 20 past every hour on 5th of July      | `CronExpressionBuilder::new().at_minutes(vec![20]).and_then(s.on_days_of_month(vec![5])).and_then(s.during_months(vec![7])).and_then(Ok(s.compile()));` | `20 * 5 7 *`       |
| Run every 5 min during office hours           | `CronExpressionBuilder::new().every_x(5, CronTimeUnit::Minute).and_then(s.at_hours(vec![9,10,11,12,13,14,15,16])).and_then(Ok(s.compile()));` | `*/5 9-16 * * *`  |
| Run at quarter past and quarter to every hour | `CronExpressionBuilder::new().every("hour").and_then(s.at_minutes(vec![15,45])).and_then(Ok(s.compile()));` | `15,45 * * * *`   |

---

## 📚 API Reference

### `CronExpressionBuilder` Methods

| Method                                       | Description                                        |
| -------------------------------------------- | -------------------------------------------------- |
| `at_minutes(minutes: Vec<i32>)`              | Set specific minutes of the hour                   |
| `at_hours(hours: Vec<i32>)`                  | Set specific hours of the day                      |
| `at_time(time: &str)`                        | Set time in `"HH:MM"` format                       |
| `every(unit: &str)`                          | Run every unit (minute, hour, day, month, weekday) |
| `every_x(interval: i32, unit: CronTimeUnit)` | Run every X units                                  |
| `on_week_days(days: Vec<i32>)`               | Set specific weekdays (0=Sunday)                   |
| `on_days_of_month(days: Vec<i32>)`           | Set specific days of month (1-31)                  |
| `during_months(months: Vec<i32>)`            | Set specific months (1-12)                         |
| `compile()`                                  | Generate final cron expression                     |

---

### `CronTimeUnit` Enum

```rust
pub enum CronTimeUnit {
    Minute,
    Hour,
    DayOfMonth,
    Month,
    DayOfWeek,
}
```

---

## 🛠 Validators

Behind the scenes, `CronValidators` ensures your fields are valid:

- `validate_minute(i32)`
- `validate_hour(i32)`
- `validate_day_of_month(i32)`
- `validate_month(i32)`
- `validate_day_of_week(i32)`
- `validate_time(&str)`

---

## 🤝 Contributing

Contributions are very welcome! Feel free to open issues or pull requests to improve this library.

---

## 📜 License

This project is licensed under the [MIT License](LICENSE).

---

# 🚀 Quick Start Example

```rust
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
```

---

# ✅ Notes

- Replace `satyajitnayk` with your actual GitHub username in badges.
- If you change the crate name (`natural-cron-rs` → `natural_cron`), update the docs URL.
- Later, you can add feature flags (e.g., `"serde"` support) if needed.
