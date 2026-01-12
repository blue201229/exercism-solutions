#[derive(Debug, PartialEq)]
pub struct Clock{
    pub hours: i32,
    pub minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {

        Self{
            hours: (hours + minutes / 60 + if minutes % 60 < 0 {-1} else {0}).rem_euclid(24),
            minutes:minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let _minutes = self.minutes + minutes;
        Self{
            hours:(self.hours + _minutes / 60 + if _minutes % 60 < 0 {-1} else {0}).rem_euclid(24), 
            minutes:(self.minutes + minutes).rem_euclid(60)
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
