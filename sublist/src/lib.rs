#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    
    if _first_list.len() == _second_list.len() {
        return equal(_first_list, _second_list);
    }
    else if _first_list.len() < _second_list.len() {
        return contained(_first_list, _second_list);
    }
    else{
        if contained(_second_list, _first_list) == Comparison::Sublist{
            return Comparison::Superlist
        }
        else{
            return Comparison::Unequal;
        }
    }

}
pub fn contained<T: PartialEq>(_smaller_list: &[T], _bigger_list: &[T]) -> Comparison {

    for (i, _ch) in _bigger_list.iter().enumerate() {
        if i + _smaller_list.len() > _bigger_list.len(){
            return Comparison::Unequal;
        }
        let mut flag = true;
        for (j, c) in _smaller_list.iter().enumerate(){
            if *c == _bigger_list[i+j] {
                continue;
            }
            else{
                flag = false;
                break;
            }
        }
        if flag == true{
            return Comparison::Sublist;
        }
    }
    return Comparison::Unequal;
}
pub fn equal<T: PartialEq>(list_a: &[T], list_b: &[T]) -> Comparison {

    if contained(list_a, list_b) == Comparison::Sublist{
        return Comparison::Equal
    }
    else{
        return Comparison::Unequal;
    }
}
