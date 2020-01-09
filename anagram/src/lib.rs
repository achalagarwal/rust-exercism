use std::collections::HashSet;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {

    // here word is a &str and not a string that means its borrowed but its a reference that means we link the same set of references back to the
    // hashset which we will create and the hashset is a transferred object


    // set the storage block

    let mut arr: [i32; 26] = [0; 26]; 

    // compute the letter count

    let non_zero_entries = word.chars().
    filter(|c| c.is_alphabetic()).
    map(|c| (c as char).to_lowercase()).
    flat_map(|x| x).
    // filter(|c| c.is_ascii_lowercase()).
    map(|c| c as u32 - 97_u32).
    fold(0,| acc ,x| if arr[x as usize] == 0 { arr[x as usize] += 1; acc+1} else { arr[x as usize] += 1; acc});

    let mut hashset: HashSet<&'a str> = HashSet::new();
 
    possible_anagrams.iter().
    filter(|x| x.to_ascii_lowercase()!=word.to_ascii_lowercase()).
    filter(|x| reduces_to_zero(x, arr, non_zero_entries)).
    fold(true ,|_, x| hashset.insert(x));

    return hashset;
}

fn reduces_to_zero(word: &str,arr:[i32; 26], non_zero_entries: u32  ) -> bool {
    let mut cloned_arr = arr.clone();

    word.chars().
    filter(|c| c.is_alphabetic()).
    map(|c| (c as char).to_lowercase()).
    flat_map(|x| x).
    // filter(|c| c.is_ascii_lowercase()).
    map(|c| c as u32 - 97_u32).
    fold_while(non_zero_entries,| acc ,x| if cloned_arr[x as usize] == 1 { cloned_arr[x as usize] -= 1; Continue(acc-1)} else if cloned_arr[x as usize] == 0  { Done(1)} else {cloned_arr[x as usize] -=1; Continue(acc)}) == 0
} 
