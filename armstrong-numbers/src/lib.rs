use std::u32;
use std::convert::TryInto;

pub fn is_armstrong_number(num: u32) -> bool {

    // let digits = get_number_of_digits(num);

    // let sum:u32 = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).map(|n|n.pow(digits)).sum();;
    let string = num.to_string();
    let sum:u32 = string.chars().map(|d| d.to_digit(10).unwrap()).map(|n|n.pow((string.len()).try_into().unwrap())).sum();
    // let iter2 = iter.chars().map(|d| d.to_digit(10).unwrap());
    // // why is this still at iterator of characters?
    // let sum:u32 = iter2.map(|n|n.pow(digits)).sum();
    // print!("{}", sum);
    return sum == num

    // unimplemented!("true if {} is an armstrong number", num)
    // let mut 
}

pub fn get_number_of_digits(num: u32) -> u32 {

    // there are quite a few ways to determine the number of digits 
    // iterate while dividing by 10
    // converting to string and returning len
    // using log base 10 
    let num_f = num as f32;

    (num_f.log10()+1.0) as u32
    
}

