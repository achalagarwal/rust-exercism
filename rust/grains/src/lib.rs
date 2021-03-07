pub fn square(s: u32) -> u64 {
    
    if s<1 || s>64 {
        panic!("Square must be between 1 and 64");
    }
    // if s>64{
    //     panic!("Square must be between 1 and 64");
    // }

    //assert!(s>0);
    let sq: u64 = 2_u64.pow(s-1);

    sq

    //unimplemented!("grains of rice on square {}", s);
}

pub fn total() -> u64 {
    // the optimal value is just 2**64 -1 which is also the max value supported by an unsigned 64 bit number ;)

    let grains: u64 = u64::max_value();
    return grains
    //unimplemented!();
    // what if we want to do it the proper way?

    // create an iterator for 1-64
    // fold (reduce) it with sum of square()s
}
