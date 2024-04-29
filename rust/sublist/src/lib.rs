#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    _first_list.is_empty()
        || _second_list
            .windows(_first_list.len())
            .any(|slice| _first_list == slice)
}

pub fn is_superlist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    is_sublist(_second_list, _first_list)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    }

    if is_superlist(_first_list, _second_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}
