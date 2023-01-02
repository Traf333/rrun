use chrono::{DateTime, FixedOffset, Local, Utc, TimeZone, NaiveTime};
use chrono::format::ParseError;
use chrono::format::StrftimeItems;

pub fn show_time(pattern: &str) {
    let local_time = Local::now();
    let list = [("Jack", -5), ("Son", 1), ("Dev team", 3)];

    println!("{}", (0..46).map(|_| "-").collect::<String>());

    println!(
        " {: <10} | {: <20} | {: <10}",
        "Who", "time", "timezone"
    );
    println!("{}", (0..46).map(|_| "-").collect::<String>());

    println!(
        " {: <10} | {: <20} | {: <10}",
        "Me",
        local_time.format_with_items(StrftimeItems::new("%Y-%m-%d %H:%M")),
        local_time.offset()
    );

    for (name, zone) in list {
        let offset = FixedOffset::east_opt(zone * 3600).unwrap();
        let some_time = local_time.with_timezone(&offset);

        println!(" {: <10} | {: <20} | {: <10}",
                 name,
                 some_time.format_with_items(StrftimeItems::new("%Y-%m-%d %H:%M")),
                 offset
        );
    }

    // let time_only = DateTime::<Utc>::parse_from_str(pattern, "%H:%M").unwrap();

    // let expected_time = Utc::now().add_hours(3);
    // let duration = expected_time - utc_time;
    // dbg!(duration.num_minutes());
    // dbg!(lwith_tz.format_with_items(fmt).to_string());
    // dbg!(expected_time.with_timezone(&rio_timezone).format_with_items(fmt).to_string());
}
