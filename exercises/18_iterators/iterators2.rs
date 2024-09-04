// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(), // return new string if empty string
        Some(first) => first.to_uppercase().to_string() + chars.as_str(),
    }
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {

    // naive implementation with for loop
    // let mut word_vec: Vec<String> = Vec::with_capacity(words.len());
    // for w in words {
    //     word_vec.push(capitalize_first(w));
    // }
    // word_vec

    // using iterators with map and collect
    words.iter()
         .map(|word| capitalize_first(word)) // closure!
         .collect::<Vec<String>>() // turbo fish to hint return type!
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    // convert the vector returned by the previous iterator
    // back into a iterable and collect into String
    // probably not very effective
    // capitalize_words_vector(words).into_iter().collect()

    // using iterators with map and collect on the raw &[&str] array
    words.iter()
         .map(|word| capitalize_first(word))
         .collect::<String>()

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
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
