use rust_extensions::date_time::DateTimeStruct;

pub fn good_as_minute_key(src: &DateTimeStruct) -> Result<(), String> {
    if src.time.min > 59 {
        return Err(format!("Invalid minute {}", src.time.min));
    }

    if src.time.hour > 23 {
        return Err(format!("Invalid hour {}", src.time.hour));
    }

    if src.day < 1 || src.day > 31 {
        return Err(format!("Invalid day {}", src.day));
    }

    if src.month < 1 || src.month > 12 {
        return Err(format!("Invalid month {}", src.month));
    }

    Ok(())
}

pub fn good_as_hour_key(src: &DateTimeStruct) -> Result<(), String> {
    if src.time.min != 0 {
        return Err(format!("Hour key must has minute as 0"));
    }

    if src.time.hour > 23 {
        return Err(format!("Invalid hour {}", src.time.hour));
    }

    if src.day < 1 || src.day > 31 {
        return Err(format!("Invalid day {}", src.day));
    }

    if src.month < 1 || src.month > 12 {
        return Err(format!("Invalid month {}", src.month));
    }

    Ok(())
}

pub fn good_as_day_key(src: &DateTimeStruct) -> Result<(), String> {
    if src.time.min != 0 {
        return Err(format!("Day key must has minute as 0"));
    }

    if src.time.hour != 0 {
        return Err(format!("Day key must has hour as 0"));
    }

    if src.day < 1 || src.day > 31 {
        return Err(format!("Invalid day {}", src.day));
    }

    if src.month < 1 || src.month > 12 {
        return Err(format!("Invalid month {}", src.month));
    }

    Ok(())
}

pub fn good_as_month_key(src: &DateTimeStruct) -> Result<(), String> {
    if src.time.min != 0 {
        return Err(format!("Month key must has minute as 0"));
    }

    if src.time.hour != 0 {
        return Err(format!("Month key must has hour as 0"));
    }

    if src.day != 0 {
        return Err(format!("Month key must has day as 0"));
    }

    if src.month < 1 || src.month > 12 {
        return Err(format!("Invalid month {}", src.month));
    }

    Ok(())
}
