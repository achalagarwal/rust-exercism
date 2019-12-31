use std::u32;

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = get_number_of_digits(num);
    (0..digits).fold((num,0), |acc,_| ((acc.0)/10,acc.1 + ((acc.0 % 10) as u32).pow(digits))).1 == num
}

pub fn get_number_of_digits(num: u32) -> u32 {
    let num_f = num as f32;
    (num_f.log10()+1.0) as u32
}

