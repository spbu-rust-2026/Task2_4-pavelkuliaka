use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(['\n', '\r']);
    println!("{}", longest_palindromic_substring(text));
}

fn longest_palindromic_substring(text: &str) -> &str {
    let mut transformed = Vec::with_capacity(2 * text.len() + 1);
    let mut offsets = Vec::with_capacity(text.len());

    for (offset, character) in text.char_indices() {
        transformed.push('#');
        transformed.push(character);
        offsets.push(offset);
    }

    if offsets.is_empty() {
        return "";
    }

    transformed.push('#');

    let length = transformed.len();
    let mut radius = vec![0usize; length];
    let (mut center, mut right) = (0usize, 0usize);
    let (mut best_center, mut best_radius) = (0usize, 0usize);

    for index in 1..length - 1 {
        let mut reach = 0;
        if index < right {
            reach = radius[2 * center - index].min(right - index);
        }

        while index + 1 + reach < length
            && index > reach
            && transformed[index + 1 + reach] == transformed[index - 1 - reach]
        {
            reach += 1;
        }

        radius[index] = reach;
        if index + reach > right {
            center = index;
            right = index + reach;
        }
        if reach > best_radius {
            best_center = index;
            best_radius = reach;
        }
    }

    let start = (best_center - best_radius) / 2;
    let end = offsets
        .get(start + best_radius)
        .copied()
        .unwrap_or(text.len());
    &text[offsets[start]..end]
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;
