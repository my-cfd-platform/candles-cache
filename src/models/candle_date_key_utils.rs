use crate::CandleDateKey;

pub struct DateTimeComponents {
    pub year: u64,
    pub month: u64,
    pub day: u64,
    pub hour: u64,
    pub minute: u64,
}

impl DateTimeComponents {
    pub fn from_date_key(src: CandleDateKey) -> Self {
        let value = src.get_value();
        let year = value / 100000000;

        let value = value - year * 100000000;

        let month = value / 1000000;

        if month > 12 {
            panic!("Invalid month {}", month);
        }

        let value = value - month * 1000000;

        let day = value / 10000;

        if day > 31 {
            panic!("Invalid day {}", day);
        }

        let value = value - day * 10000;

        let hour = value / 100;

        if hour > 23 {
            panic!("Invalid hour {}", hour);
        }

        let minute = value - hour * 100;

        if minute > 59 {
            panic!("Invalid minute {}", minute);
        }

        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }

    pub fn good_as_minute_key(&self) -> Result<(), String> {
        if self.minute > 59 {
            return Err(format!("Invalid minute {}", self.minute));
        }

        if self.hour > 23 {
            return Err(format!("Invalid hour {}", self.hour));
        }

        if self.day < 1 || self.day > 31 {
            return Err(format!("Invalid day {}", self.day));
        }

        if self.month < 1 || self.month > 12 {
            return Err(format!("Invalid month {}", self.month));
        }

        Ok(())
    }

    pub fn good_as_hour_key(&self) -> Result<(), String> {
        if self.minute != 0 {
            return Err(format!("Hour key must has minute as 0"));
        }

        if self.hour > 23 {
            return Err(format!("Invalid hour {}", self.hour));
        }

        if self.day < 1 || self.day > 31 {
            return Err(format!("Invalid day {}", self.day));
        }

        if self.month < 1 || self.month > 12 {
            return Err(format!("Invalid month {}", self.month));
        }

        Ok(())
    }

    pub fn good_as_day_key(&self) -> Result<(), String> {
        if self.minute != 0 {
            return Err(format!("Day key must has minute as 0"));
        }

        if self.hour != 0 {
            return Err(format!("Day key must has hour as 0"));
        }

        if self.day < 1 || self.day > 31 {
            return Err(format!("Invalid day {}", self.day));
        }

        if self.month < 1 || self.month > 12 {
            return Err(format!("Invalid month {}", self.month));
        }

        Ok(())
    }

    pub fn good_as_month_key(&self) -> Result<(), String> {
        if self.minute != 0 {
            return Err(format!("Month key must has minute as 0"));
        }

        if self.hour != 0 {
            return Err(format!("Month key must has hour as 0"));
        }

        if self.day != 0 {
            return Err(format!("Month key must has day as 0"));
        }

        if self.month < 1 || self.month > 12 {
            return Err(format!("Invalid month {}", self.month));
        }

        Ok(())
    }
}
