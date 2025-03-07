fn main() {
    let sentence = String::from("Rust is fast and safe.");
    let word = extract_word(&sentence, 0, 4); // Extracting "Rust"
    println!("Extracted Word: {}", word);

    // Modifying original string
    let mut sentence = sentence;
    sentence.push_str(" Indeed!");
    println!("Modified Sentence: {}", sentence);
}

fn extract_word(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]
}

