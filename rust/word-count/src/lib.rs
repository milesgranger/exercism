use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    let mut valid_chars = (b'a'..b'z').map(|v| v as char).collect::<Vec<char>>();
    valid_chars.extend_from_slice(&[' ', '\'', ',', '\n', '1', '2']);

    words
        .to_lowercase()
        .chars()
        .filter(|c| valid_chars.contains(c))
        .collect::<String>()
        .replace(',', " ")
        .split_whitespace()
        .map(|word| match word.starts_with("'") {
            true => word.replace("'", ""),
            false => word.to_owned(),
        })
        .for_each(|word| {
            *counts.entry(word).or_insert(0) += 1;
        });
    counts
}
