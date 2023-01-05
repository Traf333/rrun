use chrono::{DateTime, FixedOffset, Local, Utc, TimeZone, NaiveTime, Duration};
use chrono::format::ParseError;
use chrono::format::StrftimeItems;
use chrono::Timelike;

use std::num::ParseIntError;

fn split_and_parse(s: &str) -> Result<(u32, u32), ParseIntError> {
    let parts: Vec<&str> = s.split(':').collect();
    let h = parts[0].parse::<u32>()?;
    let m = if parts.len() > 1 { parts[1].parse::<u32>()? } else { 0 };
    Ok((h, m))
}

fn format_line(label: &str, time: String, timezone: String) {
    println!(" {: <20} | {: <20} | {: <10}", label, time, timezone);
    divider();
}

fn divider() {
    println!("{}", (0..46).map(|_| "-").collect::<String>());
}

pub fn show_time(pattern: Option<String>) {
    let local_time = Local::now();

    let (hours, minutes) = match pattern {
        Some(ref pattern) => split_and_parse(pattern.as_str()).unwrap(),
        None => (local_time.hour(), local_time.minute())
    };

    let list = [("Jack", -5), ("Son", 1), ("Dev team", 3)];

    format_line("Who", "time".to_string(), "timezone".to_string());

    #[allow(deprecated)]
        let shifted_time = Local::today().and_hms_opt(hours, minutes, 0).unwrap();

    format_line(
        "Local Time",
        local_time.format_with_items(StrftimeItems::new("%Y-%m-%d %H:%M")).to_string(),
        local_time.offset().to_string(),
    );
    if pattern.is_some() {
        format_line(
            "Planned Time",
            shifted_time.format_with_items(StrftimeItems::new("%Y-%m-%d %H:%M")).to_string(),
            local_time.offset().to_string(),
        );
    }

    for (name, zone) in list {
        let offset = FixedOffset::east_opt(zone * 3600).unwrap();
        let some_time = shifted_time.with_timezone(&offset);

        format_line(
            name,
            some_time.format_with_items(StrftimeItems::new("%H:%M")).to_string(),
            offset.to_string(),
        );
    }
}
