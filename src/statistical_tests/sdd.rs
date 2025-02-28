//! Standard Deviation of Differences (SDD) Test
//!
//! This module implements the Standard Deviation of Differences statistical test,
//! which measures the variability in the differences between consecutive characters.

use crate::statistical_tests::utils::convert_string;

/// Calculates the Standard Deviation of Differences (SDD) for the given text
///
/// The Standard Deviation of Differences measures the variability in the differences
/// between consecutive characters in the text. It's useful for distinguishing between
/// different types of ciphers based on how character values change throughout the text.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Standard Deviation of Differences value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::sdd::get_sdd;
///
/// let text = "HELLOWORLD";
/// let sdd = get_sdd(text);
/// assert!(sdd >= 0.0);
/// ```
pub fn get_sdd(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    // Calculate differences between consecutive characters
    let mut differences = Vec::with_capacity(data.len() - 1);
    for i in 1..data.len() {
        // Use modular arithmetic to handle wraparound (e.g., Z to A)
        let diff = (data[i] as isize - data[i-1] as isize).rem_euclid(26) as usize;
        differences.push(diff);
    }
    
    // Calculate mean of differences
    let sum: usize = differences.iter().sum();
    let mean = sum as f64 / differences.len() as f64;
    
    // Calculate standard deviation
    let variance: f64 = differences.iter()
        .map(|&d| {
            let diff = d as f64 - mean;
            diff * diff
        })
        .sum::<f64>() / differences.len() as f64;
    
    let std_dev = variance.sqrt();
    
    std_dev * 10.0 // Scale factor to match Python implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sdd_constant_differences() {
        // Text with constant differences between characters
        let text = "ACEGIKMOQSUWY";  // Each character is 2 positions apart
        let sdd = get_sdd(text);
        assert_eq!(sdd, 0.0);  // Standard deviation should be 0
    }

    #[test]
    fn test_sdd_varied_differences() {
        // Text with varied differences between characters
        let text = "ABCDEFZYXWVU";
        let sdd = get_sdd(text);
        assert!(sdd > 0.0);  // Standard deviation should be positive
    }

    #[test]
    fn test_sdd_short_text() {
        // Text that's too short for differences
        let text = "A";
        let sdd = get_sdd(text);
        assert_eq!(sdd, 0.0);
    }
}