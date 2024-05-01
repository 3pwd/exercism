#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn is_sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|slice| first_list == slice)
}

pub fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    is_sublist(second_list, first_list)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (
        is_sublist(first_list, second_list),
        is_superlist(first_list, second_list),
    ) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Sublist,
        (false, true) => Comparison::Superlist,
        (false, false) => Comparison::Unequal,
    }
}
