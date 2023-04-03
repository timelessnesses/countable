use chrono;

pub fn format_duration(duration: chrono::Duration) -> String {
    let mut result = String::new();
    let mut duration = duration;
    let years = duration.num_weeks() / 52;
    let months = duration.num_weeks() / 4;
    let weeks = duration.num_weeks() % 4;
    let days = duration.num_days() % 7;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    if years > 0 {
        result.push_str(&format!(
            "{} year{} ",
            years,
            if years > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::weeks(years * 52);
    }
    if months > 0 {
        result.push_str(&format!(
            "{} month{} ",
            months,
            if months > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::weeks(months * 4);
    }
    if weeks > 0 {
        result.push_str(&format!(
            "{} week{} ",
            weeks,
            if weeks > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::weeks(weeks);
    }
    if days > 0 {
        result.push_str(&format!("{} day{} ", days, if days > 1 { "s" } else { "" }));
        duration = duration - chrono::Duration::days(days);
    }
    if hours > 0 {
        result.push_str(&format!(
            "{} hour{} ",
            hours,
            if hours > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::hours(hours);
    }
    if minutes > 0 {
        result.push_str(&format!(
            "{} minute{} ",
            minutes,
            if minutes > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::minutes(minutes);
    }
    if seconds > 0 {
        result.push_str(&format!(
            "{} second{} ",
            seconds,
            if seconds > 1 { "s" } else { "" }
        ));
        duration = duration - chrono::Duration::seconds(seconds);
    }

    return result;
}
