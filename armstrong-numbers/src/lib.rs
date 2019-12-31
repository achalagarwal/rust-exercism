use std::u32;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_number_of_digits(num);
    match (0..digits).try_fold((num,0), |(current_num,sum),_| short_circuit_armstrong(sum + ((current_num % 10) as u32).pow(digits), num, current_num/10)) {
        None => false,
        Some((_, sum)) => sum == num
    }
}

#[inline]
pub fn get_number_of_digits(num: u32) -> u32 {
    let num_f = num as f32;
    (num_f.log10()+1.0) as u32
}

#[inline]
pub fn short_circuit_armstrong(sum: u32, num: u32, pass:u32) -> Option<(u32,u32)>{
    if sum > num { return None }
    Some((pass, sum))
}