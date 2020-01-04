use std::collections::HashSet;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {

    // here word is a &str and not a string that means its borrowed but its a reference that means we ink the same set of references back to the
    // hashset which we will create and the hashset is a transferred object
    // is that a good thing?
    // what if the user wants some independence 
    // just a counter works
    // a hashset is a set containing a subset of // possible_anagrams that are the anagrams of the //word

    // set the storage block

    let mut arr: [i32; 26] = [0; 26]; 

    // compute the letter count

    let non_zero_entries = word.chars().
    map(|c| c.to_ascii_lowercase() as u32 - 97_u32).
    fold(0,| acc ,x| if arr[x as usize] == 0 { arr[x as usize] += 1; acc+1} else { arr[x as usize] += 1; acc});

    // make a copy of the array and send it with each word in the possible anagram hashset and check if the 
    // nonzero entries are getting reduced to zero, if yes, then we can add that to a new hashset

    // so lets make the final hashset where we have to add

    let mut hashset: HashSet<&'a str> = HashSet::new();
 
    possible_anagrams.iter().
    filter(|x| x.to_ascii_lowercase()!=word.to_ascii_lowercase()).
    filter(|x| reduces_to_zero(x, arr, non_zero_entries)).
    fold(true ,|_, x| hashset.insert(x));

    return hashset;
    // unimplemented!(
    //     "For the '{}' word find anagrams among the following words: {:?}",
    //     word,
    //     possible_anagrams
    // );
}

fn reduces_to_zero(word: &str,arr:[i32; 26], non_zero_entries: u32  ) -> bool {
    let mut cloned_arr = arr.clone();

    word.chars().
    map(|c| c.to_ascii_lowercase() as u32 - 97_u32).
    fold_while(non_zero_entries,| acc ,x| if cloned_arr[x as usize] == 1 { cloned_arr[x as usize] -= 1; Continue(acc-1)} else if cloned_arr[x as usize] == 0  { Done(1)} else {cloned_arr[x as usize] -=1; Continue(acc)}) == 0
} 
