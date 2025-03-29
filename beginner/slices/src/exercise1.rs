fn main() {
    // Create an owned String containing the text to search within.
    let text = String::from("Today is a very warm and sunny day.");
    // Create an array of string literals (&str) representing the words to search for.
    let words = ["very", "arm", "say", "sun", "dew"];
    // Declare a mutable variable 'pos' to store the starting position (index)
    // of a found word, or the length of the text if not found.
    let mut pos;

    println!("Text: {text}");

    // Loop through each 'word' in the 'words' array.
    for word in words {
        // Call the find_substr_pos function to search for the current 'word' within the 'text'.
        // Pass immutable references (&str) to both the text and the word.
        // The function returns the starting index if found, or text.len() if not found.
        // Store the returned value in the 'pos' variable.
        pos = find_substr_pos(&text, word);

        // Check the value returned by find_substr_pos.
        // If 'pos' is equal to the length of the 'text', it means the word was not found.
        if pos == text.len() {
            // Print a message indicating the word is not present.
            println!("{word} is not present in text");
        } else {
            // Otherwise, the word was found, and 'pos' holds its starting index.
            // Print a message indicating the word was found and its index.
            println!("{word} present at index {pos}");
        }
    }
}

// This function searches for the first occurrence of a substring ('substr')
// within a given text ('text').
// It takes string slices (&str) as input for efficiency (avoids copying).
// It returns a 'usize' which is the starting index of the 'substr' if found.
// If the 'substr' is not found, it returns the length of the 'text' as a signal.
fn find_substr_pos(text: &str, substr: &str) -> usize {
    // Optimization: If the text is shorter than the substring,
    // the substring cannot possibly be present. Return the "not found" signal.
    if text.len() < substr.len() {
        return text.len(); // Return text length to indicate "not found".
    }

    // Store the length of the substring for easier access and performance.
    let len = substr.len();

    // Iterate through all possible starting positions for the substring within the text.
    // The loop variable 'start' will represent the starting index in 'text'.
    // The range goes from 0 up to the last possible index where the substring could start.
    // `text.len() - len` gives the index of the last character where the substring could start.
    // `+ 1` is needed because Rust ranges `a..b` exclude `b`, so `a..b+1` includes `b`.
    for start in 0..text.len() - len + 1 {
        // Create a slice of 'text' starting at the current 'start' index
        // and having the same length as 'substr'.
        // Compare this slice directly with the 'substr'.
        if substr == &text[start..start + len] {
            // If the substring matches the slice, we found it!
            // Immediately return the current 'start' index.
            return start;
        }
    }
    // If the loop completes without finding a match (i.e., the `return start;`
    // inside the loop was never executed), it means the substring is not present.
    // Return the length of the text as the signal for "not found".
    text.len()
}
