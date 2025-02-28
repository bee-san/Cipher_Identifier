//! Long Repeat (LR) Test
//!
//! This module implements the Long Repeat statistical test,
//! which measures the length of the longest repeated substring in the text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Long Repeat (LR) value for the given text
///
/// The Long Repeat test measures the length of the longest repeated substring in the text.
/// It's useful for identifying patterns in the ciphertext that might indicate the type of cipher used.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Long Repeat value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::lr::get_lr;
///
/// let text = "HELLOWORLD";
/// let lr = get_lr(text);
/// assert!(lr >= 0.0);
/// ```
pub fn get_lr(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    let mut longest_repeat = 0;
    
    // Check for repeats of different lengths
    for length in (2..=data.len() / 2).rev() {
        if has_repeat(&data, length) {
            longest_repeat = length;
            break;
        }
    }
    
    longest_repeat as f64
}

/// Checks if the text has a repeated substring of the given length
///
/// # Arguments
///
/// * `data` - The numeric representation of the text
/// * `length` - The length of the substring to check for repeats
///
/// # Returns
///
/// True if a repeat of the given length is found, false otherwise
fn has_repeat(data: &[usize], length: usize) -> bool {
    let mut substrings = HashMap::new();
    
    // Check all possible substrings of the given length
    for i in 0..=(data.len() - length) {
        let substring = &data[i..(i + length)];
        let key = substring.to_vec(); // Convert slice to vector for HashMap key
        
        if substrings.contains_key(&key) {
            return true;
        }
        
        substrings.insert(key, i);
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lr_with_repeats() {
        // Text with repeated substrings
        let text = "ABCDEFABCDEF";
        let lr = get_lr(text);
        assert!(lr >= 6.0); // Should find repeat of length 6
    }

    #[test]
    fn test_lr_without_repeats() {
        // Text without repeated substrings
        let text = "ABCDEFGHIJKL";
        let lr = get_lr(text);
        assert_eq!(lr, 0.0); // No repeats longer than 1 character
    }

    #[test]
    fn test_lr_short_text() {
        // Text that's too short for repeats
        let text = "A";
        let lr = get_lr(text);
        assert_eq!(lr, 0.0);
    }
}