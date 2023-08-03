use std::fmt::{self, Display, Formatter};

const MINUTES_DAY: i32 = 24 * 60;
const MINUTES_HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: ((hours * MINUTES_HOUR + minutes) % MINUTES_DAY + MINUTES_DAY) % MINUTES_DAY,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_HOUR,
            self.minutes % MINUTES_HOUR
        )
    }
}
