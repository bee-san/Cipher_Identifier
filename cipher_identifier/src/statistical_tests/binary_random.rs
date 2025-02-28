//! Binary Random Test
//!
//! This module implements the Binary Random statistical test,
//! which analyzes the randomness of binary patterns in the text.

use crate::statistical_tests::utils::convert_string;

/// Calculates the Binary Random test score for the given text
///
/// The Binary Random test analyzes the randomness of binary patterns in the text.
/// It's useful for distinguishing between different types of ciphers.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// A string indicating the result of the binary random test ("Y" or "N")
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::binary_random::get_binary_random;
///
/// let text = "HELLOWORLD";
/// let result = get_binary_random(text);
/// assert!(result == "Y" || result == "N");
/// ```
pub fn get_binary_random(text: &str) -> String {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return "N".to_string();
    }
    
    // Count transitions (changes from one character to another)
    let mut transitions = 0;
    for i in 1..data.len() {
        if data[i] != data[i-1] {
            transitions += 1;
        }
    }
    
    // Calculate the ratio of transitions to possible transitions
    let possible_transitions = data.len() - 1;
    let ratio = transitions as f64 / possible_transitions as f64;
    
    // If the ratio is above a threshold, consider it random
    if ratio > 0.45 {
        "Y".to_string()
    } else {
        "N".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_random_alternating() {
        // Alternating pattern should be considered random
        let text = "ABABABABABABABABAB";
        let result = get_binary_random(text);
        assert_eq!(result, "Y");
    }

    #[test]
    fn test_binary_random_repeated() {
        // Repeated pattern should not be considered random
        let text = "AAAAAAAAAAAAAAAAA";
        let result = get_binary_random(text);
        assert_eq!(result, "N");
    }
}