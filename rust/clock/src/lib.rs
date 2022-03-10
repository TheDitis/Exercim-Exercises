use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::format_values(hours, minutes);
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let sum_minutes = self.minutes + minutes;
        if sum_minutes < 0 || sum_minutes >= 60 {
            let (hours, minutes) = Clock::format_values(self.hours, sum_minutes);
            return Clock { hours, minutes }
        }
        Clock { hours: self.hours, minutes: sum_minutes }
    }

    fn format_values(mut hours: i32, mut minutes: i32) -> (i32, i32) {
        // Overflow minutes go to hours
        let overflow_minutes = minutes - minutes % 60;
        hours += overflow_minutes / 60;
        minutes -= overflow_minutes;
        // Ensure minutes are in range, adjusting hours if needed
        minutes = minutes % 60;
        if minutes < 0 {
            minutes = 60 + minutes;
            hours -= 1;
        }
        // Ensure hours are in range
        hours = hours % 24;
        if hours < 0 { hours = 24 + hours }

        (hours, minutes)
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = pad_zero(&self.hours.to_string());
        let minutes = pad_zero(&self.minutes.to_string());
        write!(f, "{}:{}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}


fn pad_zero(string: &String) -> String {
    if string.len() == 1 {
        ("0".to_string() + string).to_owned()
    } else {
        string.to_owned()
    }
}