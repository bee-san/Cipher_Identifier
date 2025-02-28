//! Shannon Entropy Test
//!
//! This module implements the Shannon Entropy statistical test,
//! which measures the information content or unpredictability of the text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Shannon Entropy for the given text
///
/// Shannon Entropy is a measure of the unpredictability or information content
/// in a message. Higher entropy indicates more randomness or unpredictability.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Shannon Entropy value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::shannon_entropy::get_shannon_entropy;
///
/// let text = "HELLOWORLD";
/// let entropy = get_shannon_entropy(text);
/// assert!(entropy > 0.0);
/// ```
pub fn get_shannon_entropy(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.is_empty() {
        return 0.0;
    }
    
    // Count frequency of each character
    let mut freq_map = HashMap::new();
    for &c in &data {
        *freq_map.entry(c).or_insert(0) += 1;
    }
    
    // Calculate entropy
    let text_len = data.len() as f64;
    let mut entropy = 0.0;
    
    for &count in freq_map.values() {
        let probability = count as f64 / text_len;
        entropy -= probability * probability.log2();
    }
    
    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_english_text() {
        // English text typically has entropy around 4.0-4.5
        let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let entropy = get_shannon_entropy(text);
        assert!(entropy > 3.5 && entropy < 5.0);
    }

    #[test]
    fn test_entropy_repeated_text() {
        // Repeated text has lower entropy
        let text = "AAAAAAAAAAAAAAAAAAAA";
        let entropy = get_shannon_entropy(text);
        assert!(entropy < 1.0);
    }
}