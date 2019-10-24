pub fn reverse(input: &str) -> String {

    let len = input.len();
    let mut string = String::with_capacity(len);
    let mut iter = input.chars().rev();
    iter.for_each(|c| string.push(c));
    return string;

    // input.chars().rev().fold(String::with_capacity(input.len()), |mut string, c| {
    //     string.push(c);
    //     string
    //     })
    // unimplemented!("Write a function to reverse {}", input);

    
}
