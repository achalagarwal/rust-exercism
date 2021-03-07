import time

def sum_of_multiples(limit, factors):


    all_seen = set()
    #all_seen = frozenset()

    # let sum:u32 = 0;

    for factor in factors:
        current = factor
        while current < limit :
            if not current in all_seen:
                all_seen.add(current)
                # processed.insert(current, current);
                # // unimplemented!();
                # // sum += current;
            
            current = current + factor
        
        # // unimplemented!();
    

    return sum


if __name__ == '__main__':
    a = time.time()
    sum_of_multiples(10000000, [1,2,3,5])
    b = time.time()
    print(b - a)
# fn main() {
#    let now = Instant::now();
#    // we sleep for 2 seconds
#     println!("{}", sum_of_multiples(10000000, &[1, 2,3,5]));
# //    sleep(Duration::new(2, 0));
#    // it prints '2'
#     println!("{}", now.elapsed().as_secs());
# }