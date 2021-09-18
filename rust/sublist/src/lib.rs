#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    let n1 = _first_list.len();
    let n2 = _second_list.len();

    if n1 > n2 {
        for i in 0..(n1 - n2 + 1) {
            if _second_list == &_first_list[i..(i+n2)] {
                return Comparison::Superlist;
            }
        }
    } else if n1 < n2 {
        for i in 0..(n2 - n1 + 1) {
            if _first_list == &_second_list[i..(i+n1)] {
                return Comparison::Sublist;
            }
        }
    }

    Comparison::Unequal
}
