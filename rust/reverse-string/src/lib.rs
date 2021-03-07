pub fn reverse(input: &str) -> String {

    /*
     * Approach 1: Non functional, non idiomatic
     */

    // let len = input.len();
    // let mut string = String::with_capacity(len);
    // let mut iter = input.chars().rev();
    // iter.for_each(|c| string.push(c));
    // return string;

    /*
     * Approach 2: functional, idiomatic, but uses fold() which is like a reduce and hence calls an
     * explicit function in each iteration (in this case its a string append)
     */
    //input.chars().rev().fold(String::with_capacity(input.len()), |mut string, c| {
    //    string.push(c);
    //    string
    //    })

    /*
     * Collect is an implicit version of the above fold, so should be more efficient??
     */
    
     input.chars().rev().collect()

    
}
