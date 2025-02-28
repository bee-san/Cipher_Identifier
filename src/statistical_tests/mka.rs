//! Maximum Kappa (MKA) Test
//!
//! This module implements the Maximum Kappa statistical test,
//! which measures the coincidence rate between the text and its shifts.

use crate::statistical_tests::utils::convert_string;

/// Calculates the Maximum Kappa value for the given text
///
/// The Kappa test measures the coincidence rate between the text and its shifts.
/// The maximum value is used to identify potential key lengths in polyalphabetic ciphers.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Maximum Kappa value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::mka::get_kappa;
///
/// let text = "HELLOWORLD";
/// let kappa = get_kappa(text);
/// assert!(kappa > 0.0);
/// ```
pub fn get_kappa(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    let max_shift = std::cmp::min(10, data.len() / 2);
    let mut max_kappa = 0.0;
    
    // Try different shift values
    for shift in 1..=max_shift {
        let kappa = calculate_kappa(&data, shift);
        if kappa > max_kappa {
            max_kappa = kappa;
        }
    }
    
    max_kappa
}

/// Calculates the Kappa value for a specific shift
///
/// # Arguments
///
/// * `data` - The numeric representation of the text
/// * `shift` - The shift value to use
///
/// # Returns
///
/// The Kappa value for the specified shift
fn calculate_kappa(data: &[usize], shift: usize) -> f64 {
    let mut coincidences = 0;
    let mut total = 0;
    
    // Count coincidences between the text and its shifted version
    for i in 0..(data.len() - shift) {
        if data[i] == data[i + shift] {
            coincidences += 1;
        }
        total += 1;
    }
    
    if total > 0 {
        (coincidences as f64 / total as f64) * 1000.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kappa_repeated_pattern() {
        // Text with repeated pattern should have high kappa
        let text = "ABCABCABCABC";
        let kappa = get_kappa(text);
        assert!(kappa > 100.0); // High kappa value expected
    }

    #[test]
    fn test_kappa_random_text() {
        // Random text should have lower kappa
        let text = "QWERTYUIOPASDFGHJKLZXCVBNM";
        let kappa = get_kappa(text);
        assert!(kappa < 100.0); // Lower kappa value expected
    }
}