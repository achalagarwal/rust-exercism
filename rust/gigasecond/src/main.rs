use chrono::{DateTime, Utc, TimeZone};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {

    // how about we first convert the data time object to a
    // timestamp and then add on to it 

    let timestamp_object = start.timestamp();
    
    print!("{}\n", timestamp_object);
    // and can i really add?
    let new_object = timestamp_object + 1000000000;

    print!("{}\n", new_object);
    // reconvert it back to a DateTime object and return it?
    print!("{}\n", start);  
    // return new_object.timestamp()
    return DateTime::<Utc>::from(start);
    // unimplemented!("What time is .a gigasecond later than {}", start);
}

fn main(){
    after(Utc.ymd(1977, 6, 13).and_hms(0, 0, 0));
}