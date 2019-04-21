pub fn abbreviate(phrase: &str) -> String {
    match phrase.len() {
        // No length, no acro
        0 => phrase.to_string(),

        // Has some letters
        _ => {
            // if split by whitespace, first 'word' has a colon, then that's the acro
            if phrase
                .split_whitespace()
                .into_iter()
                .take(1)
                .any(|w| w.contains(':'))
            {
                phrase
                    .split_whitespace()
                    .take(1)
                    .next()
                    .unwrap()
                    .replace(':', "")
            } else {
                // Otherwise, take each capital letter unless every char in word is uppercase
                let uppercase: Vec<char> = (b'A'..=b'Z').map(|v| char::from(v)).collect();
                phrase
                    .split_whitespace()
                    .into_iter()
                    .map(|word| {
                        // If all are uppercase we want only the first, otherwise, take each capital letter in this word
                        match word.chars().all(|c| uppercase.contains(&c)) {
                            true => word.chars().take(1).collect(),
                            false => {
                                // if all are lowercase, we want the first letter
                                match word.chars().all(|c| !uppercase.contains(&c)) {
                                    // all lowercase, we will want the first letter splitting if hyphenated
                                    true => word
                                        .split('-')
                                        .map(|w: &str| {
                                            w.chars().take(1).next().unwrap().to_ascii_uppercase()
                                        })
                                        .collect::<Vec<char>>(),

                                    // some uppercase, so we will pick those out
                                    false => {
                                        word.chars().filter(|c| uppercase.contains(c)).collect()
                                    }
                                }
                            }
                        }
                    })
                    .flat_map(|chs: Vec<char>| chs)
                    .collect()
            }
        }
    }
}
