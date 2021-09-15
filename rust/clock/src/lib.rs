use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hour = hours;
        let mut minute = minutes;

        if minute >= 60 {
            hour += minute / 60;
            minute %= 60;
        } else if minute < 0 {
            let mut dt = (minute / 60).abs();
            minute = 60 - (minute % 60).abs();
            if minute % 60 == 0 {
                minute = 0;
            } else {
                dt += 1;
            }
            hour -= dt;
        }

        if hour >= 24 {
            hour %= 24;
        } else if hour < 0 {
            hour = 24 - (hour % 24).abs();
            if hour % 24 == 0 {
                hour = 0;
            }
        }

        Clock{hour, minute}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hour, self.minute + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
