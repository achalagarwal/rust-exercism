pub fn verse(n: u32) -> String {
    // unimplemented!("emit verse {}", n)
    if n == 2{
        return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }
    else if n == 1{
        return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }
    else if n == 0 {
        return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    else{
        // let mut r = String::with_capacity(s.len());
        let example = "99 bottles of beer on the wall, 99 bottles of beer.\nTake one down and pass it around, 98 bottles of beer on the wall.\n";
        let mut r = String::with_capacity(example.len());
        r.push_str(&n.to_string());
        r.push_str(" bottles of beer on the wall, ");
        r.push_str(&n.to_string());
        r.push_str(" bottles of beer.\nTake one down and pass it around, ");
        r.push_str(&((n-1).to_string()));
        r.push_str(" bottles of beer on the wall.\n");
        return r;
    }

}


/*

2 bottles of beer on the wall, 2 bottles of beer.
Take one down and pass it around, 1 bottle of beer on the wall.

1 bottle of beer on the wall, 1 bottle of beer.
Take it down and pass it around, no more bottles of beer on the wall.

No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.

*/
pub fn sing(start: u32, end: u32) -> String {

// start to end including both
// start is the bigger number
// so I cant range from higher to lower, thus I need to range end to start and then reverse
// first I need to include all the verses ranging from the highest to the lowest + 1
// to do this I must create an iterator from lowest+1 to highest i.e. range(low+1, high_1)
// then append the last verse
    let mut song = String::new();
    for n in (end+1..start+1).rev() {

        println!("LINE {}", n);
        song = song + &verse(n);
        song = song + "\n";
        // unimplemented!();
    }
    song = song + &verse(end);
    return song;
    // unimplemented!("sing verses {} to {}, inclusive", start, end)
}
