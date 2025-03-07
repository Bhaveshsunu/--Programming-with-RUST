// Function that extracts the first word as a string slice
fn extract_word(sentence: &str) -> &str {
    // Find the first space character to determine the end of the word.
    if let Some(space_index) = sentence.find(' ') {
        &sentence[..space_index]
    } else {
        sentence
    }
}

fn main() {
    // Original sentence input by the user.
    let mut sentence = String::from("Rust is fast and safe.");
    
    // Extract the word "Rust" as a slice.
    let word_slice = extract_word(&sentence);
    
    // Clone the slice to create an owned String.
    // This ensures the extracted word remains valid after modifying the original sentence.
    let extracted_word = word_slice.to_string();
    println!("Extracted word: {}", extracted_word);
    
    // Modify the original string.
    sentence.push_str(" Let's build great software!");
    println!("Modified sentence: {}", sentence);
    
    // The extracted word remains unchanged.
    println!("Extracted word after modification: {}", extracted_word);
}

