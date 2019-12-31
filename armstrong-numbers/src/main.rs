mod lib;
// use get_number_of_digits;

fn main(){

    let v = (0..10000000).map(|x| lib::is_armstrong_number(x)).filter(|x| *x ==  true).count();
    print!("{}", v);
    let d = lib::get_number_of_digits(10000);
    assert_eq!(d, 5);
    let d = lib::get_number_of_digits(10001);
    assert_eq!(d, 5);
    let d = lib::get_number_of_digits(999);
    assert_eq!(d, 3);
    let d = lib::get_number_of_digits(7);
    assert_eq!(d, 1);
    let d = lib::get_number_of_digits(0);
    assert_eq!(d, 0);
    let d = lib::is_armstrong_number(9_926_315);
    assert_eq!(d, true);
}