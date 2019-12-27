// use chrono::{DateTime, Utc, TimeZone};
// // use chrono::offset::TimeZone;
// // Returns a Utc DateTime one billion seconds after start.
// pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {

//     // how about we first convert the data time object to a
//     // timestamp and then add on to it 

//     let timestamp_object = start.timestamp();
    
//     print!("{}", timestamp_object);
//     // and can i really add?
//     let new_object = timestamp_object + 1000000000;

//     print!("{}", new_object);
//     // reconvert it back to a DateTime object and return it?
//     print!("{}", start);  
//     // return new_object.timestamp()
//     return start;
//     // unimplemented!("What time is .a gigasecond later than {}", start);
// }

// fn main(){

//     let dt = Utc.ymd(1977, 6, 13).and_hms_micro(10, 30, 9, 453_829);
//     after(dt);
//     // after(Utc.ymd(1977, 6, 13).and_hms(0, 0, 0));
// }

use chrono::{DateTime, Utc, TimeZone, FixedOffset, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {

    // how about we first convert the data time object to a
    // timestamp and then add on to it 

    // let timestamp_object = start.timestamp();
    
    // and can i really add?

    // let offset = FixedOffset::east(1_000_000_000);
    let offset = Duration::seconds(1_000_000_000);
    // yes we can directly add to a datetime object
    // let shifted = start + offset;
    // print!("{}", shifted);

    // reconvert it back to a DateTime object and return it?
    // return new_object.timestamp()
    // return Utc.timestamp(new_object,0);
    // shifted
    start + offset


}

// fn main(){
//     after(Utc.ymd(1977, 6, 13).and_hms(0, 0, 0));
// }