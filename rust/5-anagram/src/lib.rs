use std::collections::HashSet;

pub fn lowercase_sort(s: &str) -> [Vec<char>; 2] {
    let lower_s = s.to_lowercase().chars().collect::<Vec<char>>();
    let mut lower_and_sorted_s = lower_s.clone();
    lower_and_sorted_s.sort_unstable();

    [lower_s, lower_and_sorted_s]
}

pub fn is_anagram(l1: &Vec<char>, ls1: &Vec<char>, w2: &str) -> bool {
    let [l2, ls2] = lowercase_sort(w2);
    l1 != &l2 && ls1 == &ls2
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let [l_word, ls_word] = lowercase_sort(word);

    possible_anagrams
        .iter()
        .filter(|w| is_anagram(&l_word, &ls_word, w))
        .copied() // converts an iterator of references (&T) to an iterator of values (T)
        .collect()
}
