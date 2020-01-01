use std::char;
use std::iter;
use itertools::Itertools;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {

    plain.chars()
    .filter_map(extract_valid_char)
    .chunks(5)
    .into_iter()
    .flat_map(|x| iter::once(' ').chain(x))
    .skip(1)
    .collect::<String>()

}

pub fn encode1(plaintext: &str) -> String {
    plaintext.chars()
        .filter_map(extract_valid_char).collect::<Vec<_>>()
        .chunks(5).collect::<Vec<_>>()
        .join(&' ').iter().cloned().collect()
}


/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    
    cipher.chars()
    .filter_map(extract_valid_char)
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