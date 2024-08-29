use std::{cmp::Ordering, fmt::Display};

use chrono::TimeDelta;


#[derive(Copy, Clone, Debug)]
pub(crate) struct SinceString {
    pub seconds: u32,
    pub minutes: u32,
    pub hours: u32,
    pub days: u32,
    pub months: u32,
    pub years: u32,
}

impl SinceString {

    #[inline]
    pub fn is_zero(&self) -> bool {
        (self.seconds + self.minutes + self.hours + self.days + self.months + self.years) == 0
    }
}

impl From<TimeDelta> for SinceString {

    fn from(value: TimeDelta) -> Self {
        let seconds = value.num_seconds();
        let minutes = seconds / 60;
        let seconds = (seconds % 60) as u32;
        let hours = minutes / 60;
        let minutes = (minutes % 60) as u32;
        let days = hours / 24;
        let hours = (hours % 24) as u32;
        let months = days / 30;
        let days = (days % 30) as u32;
        let years = (months / 12) as u32;
        let months = (months / 12) as u32;
        Self {seconds, minutes, hours, days, months, years}
    }
}

impl Display for SinceString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            return write!(f, "just now");
        }

        let mut res = String::new();
        match self.years.cmp(&1) {
            Ordering::Equal => res += "1 year, ",
            Ordering::Greater => res += &format!("{} years, ", self.years),
            Ordering::Less => (),
        }
        match self.months.cmp(&1) {
            Ordering::Equal => res += "1 month, ",
            Ordering::Greater => res += &format!("{} months, ", self.months),
            Ordering::Less => (),
        }
        match self.days.cmp(&1) {
            Ordering::Equal => res += "1 day, ",
            Ordering::Greater => res += &format!("{} days, ", self.days),
            Ordering::Less => (),
        }
        if self.years + self.months == 0 {
            match self.hours.cmp(&1) {
                Ordering::Equal => res += "1 hour, ",
                Ordering::Greater => res += &format!("{} hours, ", self.hours),
                Ordering::Less => (),
            }
            match self.minutes.cmp(&1) {
                Ordering::Equal => res += "1 minute, ",
                Ordering::Greater => res += &format!("{} minutes, ", self.minutes),
                Ordering::Less => (),
            }
            if self.hours == 0 && self.minutes < 20 {
                match self.seconds.cmp(&1) {
                    Ordering::Equal => res += "1 second, ",
                    Ordering::Greater => res += &format!("{} seconds, ", self.seconds),
                    Ordering::Less => (),
                }
            }
        }

        res = res.trim().to_string().trim_end_matches(',').to_string();
        write!(f, "{} ago", res)
    }
}
