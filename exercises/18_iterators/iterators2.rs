// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

use std::vec;

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let capi = first.to_uppercase().collect::<String>();
            let rema = chars.flat_map(|c| c.to_lowercase()).collect::<String>();

            capi + &rema
        }
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // ???
    println!("This is a test");
    let mut v = Vec::<String>::new();

    for word in words.iter() {
        let s = capitalize_first(word);
        dbg!(&s);

        v.push(s);
    }
    dbg!(&v);
    v
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let mut v = Vec::<String>::new();

    for word in words.iter() {
        v.push(capitalize_first(word));
    }

    v.concat()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        println!("test_iterate_string_vec");
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
