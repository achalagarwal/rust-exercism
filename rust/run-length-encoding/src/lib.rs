pub fn encode(source: &str) -> String {
    unimplemented!("Return the run-length encoding of {}.", source);
}

pub fn decode(source: &str) -> String {

    // this is easy
    // while there is a number
    // go ahead and knock yourself out

    let s = String::from("");

    source.chars().fold((0, s), |(rl, mut string), l| if l.is_digit(10) {(rl*10 + (l as usize), string)} else if rl == 0 {string.push(l); (0, string)} else {string.push_str(&(l.to_string()).repeat(rl)); (0, string)}).1.to_string()

    // unimplemented!("Return the run-length decoding of {}.", source);
}


