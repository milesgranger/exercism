use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    if candidate.len() == 0 {
        return true;
    } else {

        let mut unique_chars = HashSet::new();

        !candidate
            .replace(' ', "")
            .replace('-', "")
            .to_lowercase()
            .chars()
            .into_iter()
            .map(|c| {
                if unique_chars.contains(&c) {
                    false  // cannot
                } else {
                    unique_chars.insert(c);
                    true
                }
            })
            .any(|r| r == false)
    }
}
