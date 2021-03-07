/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {

    // do we need to use an index to compare characters
    // at the same offset
    // is there a way to iterator through two iterables
    // at the same time?
    // if we couple them together, then it takes two iterations
    // which is not efficient
    if s1.len() != s2.len() {
        return None
    }

    //TODO
    // let mut s2_chars = s2.chars();
    // Optimize this to not use the chars nth on the str2
    // can we index in O(1)
    Some(s1.chars().fold((0,0),|(acc, index), x| if s2.chars().nth(index).unwrap() == x {(acc, index+1)} else {(acc+1,index+1)}).0)
    // unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
}

