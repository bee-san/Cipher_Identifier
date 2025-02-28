//! Repeat Order Distribution (ROD) Test
//!
//! This module implements the Repeat Order Distribution statistical test,
//! which measures the distribution of repeated characters in the text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Repeat Order Distribution (ROD) value for the given text
///
/// The Repeat Order Distribution test measures the distribution of repeated characters
/// in the text. It's useful for distinguishing between different types of ciphers
/// based on how characters repeat throughout the text.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Repeat Order Distribution value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::rod::get_rod;
///
/// let text = "HELLOWORLD";
/// let rod = get_rod(text);
/// assert!(rod > 0.0);
/// ```
pub fn get_rod(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    // Count the first occurrence of each character
    let mut first_occurrences = HashMap::new();
    for (i, &c) in data.iter().enumerate() {
        first_occurrences.entry(c).or_insert(i);
    }
    
    // Calculate the average distance between repeated characters
    let mut total_distance = 0.0;
    let mut count = 0;
    
    for (i, &c) in data.iter().enumerate() {
        if let Some(&first_pos) = first_occurrences.get(&c) {
            if i > first_pos {
                // This is a repeat of a character
                total_distance += (i - first_pos) as f64;
                count += 1;
            }
        }
    }
    
    if count == 0 {
        return 0.0;
    }
    
    // Return the average distance
    total_distance / count as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rod_with_close_repeats() {
        // Text with characters that repeat close together
        let text = "AABBCCDD";
        let rod = get_rod(text);
        assert_eq!(rod, 1.0); // Each repeat is 1 position away
    }

    #[test]
    fn test_rod_with_distant_repeats() {
        // Text with characters that repeat far apart
        let text = "ABCDABCD";
        let rod = get_rod(text);
        assert_eq!(rod, 4.0); // Each repeat is 4 positions away
    }

    #[test]
    fn test_rod_no_repeats() {
        // Text with no repeated characters
        let text = "ABCDEFG";
        let rod = get_rod(text);
        assert_eq!(rod, 0.0); // No repeats
    }
}