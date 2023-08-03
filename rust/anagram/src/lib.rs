use std::collections::HashSet;

const CASE_MASK: u8 = b'a' ^ b'A';

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for anagram in possible_anagrams {
        if is_anagram(&word, anagram) {
            set.insert(anagram.to_owned());
        }
    }

    set
}

fn is_anagram(word: &str, anagram: &str) -> bool {
    if word.to_lowercase() == anagram.to_lowercase() {
        return false;
    }

    let word = get_sorted_vec_chars(word);
    let anagram = get_sorted_vec_chars(anagram);

    word == anagram
}

// fn str_bytes_sum(word: &str) -> u8 {
//     word.bytes()
//         .fold(0, |acc, byte| acc.overflowing_add(byte).0)
// }

fn get_sorted_vec_chars(word: &str) -> Vec<char> {
    let mut chars = word.chars().fold(vec![], |mut acc, char| {
        acc.push(char_lowercase(char));
        acc
    });

    chars.sort_unstable();
    chars.into_iter().collect::<Vec<char>>()
}

fn char_lowercase(c: char) -> char {
    if c.is_ascii_uppercase() {
        (c as u8 ^ CASE_MASK) as char
    } else if c.is_uppercase() {
        c.to_lowercase().next().unwrap()
    } else {
        c
    }
}
