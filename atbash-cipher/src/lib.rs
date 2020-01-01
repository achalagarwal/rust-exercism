use std::char;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {

    plain.chars()
    .filter(|x| x.is_ascii_alphanumeric())
    .map(|x| apply_cipher_alphanumeric(x))
    .enumerate()
    .map(|(i, x)| 
        if i%5 == 0 && i>0 {let mut s = String::from(" "); s.push(x); s} 
        else{x.to_string()})
    .collect::<String>()

}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {

    cipher.chars()
    .filter(|x| x.is_ascii_alphanumeric())
    .map(|x| apply_cipher_alphanumeric(x))
    .collect::<String>()   

}

fn apply_cipher_alphanumeric(c: char) -> char {
    // assert_eq!(c.is_ascii_alphanumeric(), true);

    if c.is_ascii_alphabetic(){
        return char::from_u32(219 - c.to_ascii_lowercase() as u32).unwrap()
    }
    // else
    c
}