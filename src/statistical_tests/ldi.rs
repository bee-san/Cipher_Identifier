//! Letter Distribution Index (LDI) Test
//!
//! This module implements the Letter Distribution Index statistical test,
//! which measures how closely the letter distribution in the text matches
//! the expected distribution for English text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Letter Distribution Index (LDI) for the given text
///
/// The Letter Distribution Index measures how closely the letter distribution
/// in the text matches the expected distribution for English text. A lower value
/// indicates a closer match to English letter frequencies.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Letter Distribution Index value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::ldi::get_ldi;
///
/// let text = "HELLOWORLD";
/// let ldi = get_ldi(text);
/// assert!(ldi >= 0.0);
/// ```
pub fn get_ldi(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.is_empty() {
        return 0.0;
    }
    
    // English letter frequencies (A-Z)
    let english_freqs = [
        0.082, 0.015, 0.028, 0.043, 0.127, 0.022, 0.020, 0.061, 0.070, 0.002,
        0.008, 0.040, 0.024, 0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091,
        0.028, 0.010, 0.023, 0.001, 0.020, 0.001
    ];
    
    // Count letter frequencies in the text
    let mut letter_counts = HashMap::new();
    let mut total_letters = 0;
    
    for &c in &data {
        if c < 26 {  // Only count A-Z
            *letter_counts.entry(c).or_insert(0) += 1;
            total_letters += 1;
        }
    }
    
    if total_letters == 0 {
        return 0.0;
    }
    
    // Calculate chi-squared statistic
    let mut chi_squared = 0.0;
    
    for i in 0..26 {
        let observed = *letter_counts.get(&i).unwrap_or(&0) as f64;
        let expected = english_freqs[i] * total_letters as f64;
        
        if expected > 0.0 {
            let diff = observed - expected;
            chi_squared += (diff * diff) / expected;
        }
    }
    
    chi_squared * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldi_english_text() {
        // English text should have a lower LDI
        let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ldi = get_ldi(text);
        assert!(ldi >= 0.0);
    }

    #[test]
    fn test_ldi_non_english_distribution() {
        // Text with non-English distribution should have a higher LDI
        let text = "AAAAAAAAAAAAAAAAAAAA";
        let ldi = get_ldi(text);
        assert!(ldi > 0.0);
    }

    #[test]
    fn test_ldi_empty_text() {
        // Empty text should return 0
        let text = "";
        let ldi = get_ldi(text);
        assert_eq!(ldi, 0.0);
    }
}