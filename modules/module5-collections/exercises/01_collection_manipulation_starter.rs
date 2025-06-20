use std::collections::HashMap;

fn main() {
    // Sample text for analysis
    let text = "Rust is a multi-paradigm, general-purpose programming language. \
                Rust emphasizes performance, type safety, and concurrency. \
                Rust enforces memory safety—that is, that all references point \
                to valid memory—without requiring the use of a garbage collector \
                or reference counting present in other memory-safe languages.";

    // TODO: 1. Split the text into words and store them in a vector
    let words: Vec<&str> = text.split_whitespace().collect();

    // TODO: 2. Count the frequency of each word and store in a HashMap
    let word_counts: HashMap<&str, u32> = words.iter()
        .fold(HashMap::new(), |mut map, &word| {
            *map.entry(word).or_insert(0) += 1;
            map
        });

    // TODO: 3. Find the longest word in the text
    let longest_word = words.iter()
        .max_by_key(|word| word.len())
        .unwrap_or(&"");

    // TODO: 4. Convert all words to uppercase and store in a new vector
    let uppercase_words: Vec<String> = text.split_whitespace().map(|w| w.to_uppercase()).collect();

    // TODO: 5. Filter out words shorter than 4 characters
    let filtered_words: Vec<&&str> = words.iter()
        .filter(|word| word.len() >= 4)
        .collect();

    // Print results
    println!("Word counts: {:?}", word_counts);
    println!("Longest word: {}", longest_word);
    println!("Uppercase words: {:?}", uppercase_words);
    println!("Words longer than 3 characters: {:?}", filtered_words);
}