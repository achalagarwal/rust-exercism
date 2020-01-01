use std::char;
use std::iter;
use itertools::Itertools;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {

    let string = plain.chars()
    .filter_map(|x| extract_valid_char(x))
    .chunks(5)
    .into_iter()
    .flat_map(|x| x.chain(iter::once(' ')))
    // .map(|(i, x)| 
    //     if i%5 == 0 && i>0 {let mut s = String::from(" "); s.push(x); s} 
    //     else{x.to_string()})
    .collect::<String>();

    string[0..string.len()-1].to_string()

}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {

    cipher.chars()
    .filter_map(|x| extract_valid_char(x))
    .collect::<String>()   

}

fn extract_valid_char(c: char) -> Option<char> {
    if c.is_ascii_alphanumeric(){
        return apply_cipher_alphanumeric(c)
    }
    return None
}

fn apply_cipher_alphanumeric(c: char) -> Option<char> {
    // assert_eq!(c.is_ascii_alphanumeric(), true);

    if c.is_ascii_alphabetic(){
        return char::from_u32(219 - c.to_ascii_lowercase() as u32)
    }
    // else
    Some(c)
}