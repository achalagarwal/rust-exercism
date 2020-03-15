use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::time::{Duration, Instant};

mod lib;


fn get_rand_string(len:usize) -> String {
    thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .collect()
}
pub fn main(){

    let strings: Vec<String> = (30..10000).map(|x| get_rand_string(x)).collect();
    let mut iter = strings.iter();
    let mut now = Instant::now();
    iter.map(|x| lib::encode(&x.to_string())).fold(0, |acc, x| 1+acc);
    println!("{}\n", now.elapsed().as_millis());
    iter = strings.iter();
    now = Instant::now();
    iter.map(|x| lib::encode1(&x.to_string())).fold(0, |acc, x| 1+acc);
    println!("{}", now.elapsed().as_millis());
    return;
    print!("{}\n", '»' as u32);
    print!("{}\n", ' ' as u32);
    print!("{}\n", 'a' as u32);
    print!("{}\n", 'A' as u32);
    print!("{}\n", '0' as u32);
    print!("{}\n", '1' as u32);
    print!("{}\n", '2' as u32);
    print!("{}\n", '9' as u32);
    print!("{}\n", 'x' as u32);
    print!("{}\n", 'y' as u32);
    print!("{}\n", 'z' as u32);
    print!("{}\n", 'Y' as u32);
    print!("{}\n", 'Z' as u32);
    let z = 'a'.to_ascii_uppercase();
    print!("{}\n", z);

}