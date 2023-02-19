use std::collections::HashMap;

// write function that takes in a string and counts the number of bigrams in the string and returns a hashmap of the bigrams and their counts as key value pairs
pub fn count_bigrams(input: String) -> HashMap<String, u32> {
    let mut bigrams = HashMap::new();
    let mut chars = input.chars();
    let mut prev = chars.next();
    for c in chars {
        if let Some(p) = prev {
            let bigram = format!("{}{}", p, c);
            let count = bigrams.entry(bigram).or_insert(0);
            *count += 1;
        }
        prev = Some(c);
    }
    bigrams
}
