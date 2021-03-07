// use std::collections::HashMap;
use std::time::{Instant};
// use std::thread::sleep;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    // there are two ways to go about this, first we can create a set to check if a number has been processed and 

    // let mut processed = HashMap::new();
    let mut processed: Box<[i32]> = Box::new([0; 10000000]);

    // let unmut unprocessed: Vec =

    // let mut processed: [u32; 10000000] = [0; 10000000];


    let sum:u32 = 0;

    for factor in factors {
        let  mut current = *factor;
        while current < limit {
            
            if processed[current as usize] == 0 {
                processed[current as usize] = 1;
                // unimplemented!();
                // sum += current;
            }
            current = current + factor;
        }
        // unimplemented!();
    }

    return sum;

    // unimplemented!(
    //     "Sum the multiples of all of {:?} which are less than {}",
    //     factors,
    //     limit
    // )
}

fn main() {
   let now = Instant::now();
   // we sleep for 2 seconds
    sum_of_multiples(10000000, &[1, 2,3,5]);
//    sleep(Duration::new(2, 0));
   // it prints '2'
    println!("{}", now.elapsed().as_secs());
}