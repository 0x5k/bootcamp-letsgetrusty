// fn main() {
//     // Slices are a way to reference a contiguous sequence of elements in a collection,
//     // rather than the entire collection itself.
//     //They are a view into the data, and do not own the data they point to.

//     let slice_str = String::from("This should be a long sentence for slices showcase");
//     // reference works for both String and &str

//     let trimmed_slice: &str = trim_slice(&slice_str);
//     println!("{trimmed_slice}");
//     //println!("{:?}", trim_slice);

//     let arr = [1, 2, 3, 4, 5, 6, 7];
//     let slice_arr = &arr[..3];
//     println!("{:?}", slice_arr);
// }

// fn trim_slice(slice_str: &str) -> &str {
//     &slice_str[..14] // start of slice ragne [0..], all slice range [..]
// }

fn main() {
    let text = String::from("Today is a very warm and sunny day.");
    let words = ["very", "arm", "say", "sun", "dew"];
    let mut pos;

    println!("Text: {text}");
    for word in words {
        pos = find_substr_pos(&text, word);
        if pos == text.len() {
            println!("{word} is not present in text");
        } else {
            println!("{word} present at index {pos}");
        }
    }
}

fn find_substr_pos(text: &str, substr: &str) -> usize {
    if text.len() < substr.len() {
        return text.len();
    }
    let len = substr.len();
    for start in 0..text.len() - len + 1 {
        if substr == &text[start..start + len] {
            return start;
        }
    }
    text.len()
}
