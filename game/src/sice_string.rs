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

impl ToString for SinceString {

    fn to_string(&self) -> String {
        if self.is_zero() {
            return "just now".to_string();
        }

        let mut res = String::new();
        if self.years == 1 {
            res += "1 year, ";
        } else if self.years > 1 {
            res += &format!("{} years, ", self.years);
        }
        if self.months == 1 {
            res += "1 month, ";
        } else if self.months > 1 {
            res += &format!("{} months, ", self.months);
        }
        if self.days == 1 {
            res += "1 day, ";
        } else if self.days > 1 {
            res += &format!("{} days, ", self.days);
        }
        if self.years == 0 {
            if self.hours == 1 {
                res += "1 hour, ";
            } else if self.hours > 1 {
                res += &format!("{} hours, ", self.hours);
            }
            if self.minutes == 1 {
                res += "1 minute, ";
            } else if self.minutes > 1 {
                res += &format!("{} minutes, ", self.minutes);
            }
            if self.hours == 0 && self.minutes < 20 {
                if self.seconds == 1 {
                    res += "1 second, ";
                } else if self.seconds > 1 {
                    res += &format!("{} seconds, ", self.seconds);
                }
            }
        }

        res = res.trim().to_string().trim_end_matches(',').to_string();
        format!("{} ago", res)
    }
}
