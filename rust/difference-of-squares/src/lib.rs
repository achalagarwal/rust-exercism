pub fn square_of_sum(n: u32) -> u32 {
    // unimplemented!("square of sum of 1...{}", n)
    // this is also common knowledge

    // did i square n*(n+1)/2 correctly 
    return (n*n*n*n + n*n + 2*n*n*n)/4;
}

pub fn sum_of_squares(n: u32) -> u32 {
    // unimplemented!("sum of squares of 1...{}", n)

    // sadly I know that summation n square is
    // n n+1 2n+1 / 6

    // sorry for the brackets, I am scared of different languages parsing it differently
    return (n*(n+1)*((2*n) + 1))/6;
}

pub fn difference(n: u32) -> u32 {

    // the overhead in the call stack is real!

    // should I call the functions and subtract or should I just subtract them mathematically?

    // meh, I am lazy

    return square_of_sum(n) - sum_of_squares(n);

    // unimplemented!(
    //     "difference between square of sum of 1...{n} and sum of squares of 1...{n}",
    //     n = n,
    // )
}
