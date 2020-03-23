// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


// use std::slice::s
use std::iter;
extern crate itertools;
// use itertools::Itertools;
// use itertools::multizip;
use itertools::*;
#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RowPattern{
    PUP,
    PSP,
    SSP,
    PUS,
    SUP,
    SUS,
    SSS,
    // Undefined
}

// we need to split the string on new lines 
// for each line (row), we need to split them into categories/enums

// the jigsaw patterns are (horizontal patterns)
// 1. pipe, under, pipe
// 2. pipe, space, pipe
// 3. space, space, pipe, 
// 4. pipe, under, space
// 5. space, under, pipe
// 6. space, under, space
// 7. space, space, space

// the numbers based on jigsaw patterns: (stacked vertically)
// 1 -> 3,3,3,7
// 2 -> 6,5,4,7
// 3 -> 6,5,5,7
// 4 -> 7,1,3,7
// 5 -> 6,4,5,7
// 6 -> 6,4,1,7
// 7 -> 6,3,3,7
// 8 -> 6,1,1,7
// 9 -> 6,1,5,7
// 0 -> 6,2,1,7

// so we split and then chunk
// pub fn convert(input: &str) -> Result<String, Error> {
pub fn convert(input: &str) -> String {

    let a = input.split('\n');
    let b = a.map(|iter| convert_row_to_patterns(iter.chars().collect()));
    let c = b.collect::<Vec<_>>();
    let d = itertools::multizip((&c[0],&c[1],&c[2],&c[3]));
    let e = d.map(|(f,s,t,_):(&RowPattern,&RowPattern,&RowPattern,&RowPattern)| getCharacterFromRowPatterns(&[*f,*s,*t]));
    e.collect::<String>()
    
    // let cols : Vec<Iterator<Item=RowPattern>> = input.split('\n').map(|iter| convert_row_to_patterns(iter.chars().collect())).collect()::Vec<_>;
    // itertools::multizip(cols).map(|(f,s,t,b):(RowPattern,RowPattern,RowPattern,RowPattern)| getCharacterFromRowPatterns(&[f,s,t])).collect()
    
    // itertools::multizip(
    //     input.split('\n').
    //     map(|iter| convert_row_to_patterns(iter.chars().collect())).
    //     collect()).
    // map(|(f,s,t,b):(RowPattern,RowPattern,RowPattern,RowPattern)| getCharacterFromRowPatterns(&[f,s,t])).
    // collect()

    // let iter = input.chars();
    // iter.is_ascii()
    // .Split::split();
    // unimplemented!("Convert the input '{}' to a string", input);
}

pub fn convert_row_to_patterns(input: Vec<char>) -> Vec<RowPattern> {

    input.chunks_exact(3).map(|[f,s,r]| getRowPattern([*f,*s,*r])).collect()

}

pub fn getRowPattern(triplet: [char; 3]) -> RowPattern {
    match triplet {
        ['|','_','|'] => RowPattern::PUP,
        ['|',' ','|'] => RowPattern::PSP,
        [' ',' ','|'] => RowPattern::SSP,
        ['|','_',' '] => RowPattern::PUS,
        [' ','_','|'] => RowPattern::SUP,
        [' ','_',' '] => RowPattern::SUS,
        [' ',' ',' '] => RowPattern::SSS,
        [_,_,_] => RowPattern::SSS,
        _ => RowPattern::SSS,
        // [_,_] => RowPattern::SSS,
        // [_] => RowPattern::SSS,
        // [] => RowPattern::SSS
    }
  
}

pub fn getCharacterFromRowPatterns(triplet: &[RowPattern; 3]) -> char {

    match triplet {
        [RowPattern::SSP,RowPattern::SSP,RowPattern::SSP] => '1',
        [RowPattern::SUS,RowPattern::SUP,RowPattern::PUS] => '2',
        [RowPattern::SUS,RowPattern::SUP,RowPattern::SUP] => '3',
        [RowPattern::SSS,RowPattern::PUP,RowPattern::SSP] => '4',
        [RowPattern::SUS,RowPattern::PUS,RowPattern::SUP] => '5',
        [RowPattern::SUS,RowPattern::PUS,RowPattern::PUP] => '6',
        [RowPattern::SUS,RowPattern::SSP,RowPattern::SSP] => '7',
        [RowPattern::SUS,RowPattern::PUP,RowPattern::PUP] => '8',
        [RowPattern::SUS,RowPattern::PUP,RowPattern::SUP] => '9',
        [RowPattern::SUS,RowPattern::PSP,RowPattern::PUP] => '0',
        // None => expr,
    }
}
