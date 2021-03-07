use std::fmt;
use math::round;

#[derive(PartialEq, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}
// as hours and minutes are tightly coupled, just storing the minutes would have been a cleaner solution
// but because I saw someone's similar solution, I decided to implement it with hours and minutes instead

impl Clock {    
    pub fn new(hours: i32, minutes: i32) -> Self {
        
        let hours = (hours + (round::floor(minutes as f64/60.0,0) as i32)).rem_euclid(24);
        let minutes = minutes.rem_euclid(60);
        
        Clock{
            hours: hours,
            minutes: minutes
        }
        
    }
        
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, minutes+self.minutes )
    }              
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

