//! Index of Coincidence (IoC) Test
//!
//! This module implements the Index of Coincidence statistical test,
//! which measures the probability of two randomly selected characters being the same.

use crate::statistical_tests::utils::convert_string;

/// Calculates the Index of Coincidence (IoC) for the given text
///
/// The Index of Coincidence is a measure of the probability that two randomly
/// selected characters from the text are the same. It's useful for distinguishing
/// between different types of ciphers.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Index of Coincidence multiplied by 1000
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::ioc::get_ioc;
///
/// let text = "HELLOWORLD";
/// let ioc = get_ioc(text);
/// assert!(ioc > 0.0);
/// ```
pub fn get_ioc(text: &str) -> f64 {
    let data = convert_string(text);
    let cipher_symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#0123456789";
    let num_symbols = cipher_symbols.len();
    
    let mut counts = vec![0; num_symbols];
    let text_len = data.len();
    
    // Count occurrences of each character
    for &c in &data {
        counts[c] += 1;
    }
    
    // Calculate IoC
    let mut sum = 0.0;
    for count in counts {
        sum += count as f64 * (count as f64 - 1.0);
    }
    
    // Avoid division by zero
    if text_len <= 1 {
        return 0.0;
    }
    
    let ic = sum / (text_len as f64 * (text_len as f64 - 1.0));
    ic * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ioc_english_text() {
        // English text typically has an IoC around 1.73 (or 1730 after multiplying by 1000)
        let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let ioc = get_ioc(text);
        assert!(ioc > 1000.0 && ioc < 2000.0);
    }

    #[test]
    fn test_ioc_random_text() {
        // Random text typically has a lower IoC
        let text = "XQZJKVBPWMFYORGHLDSETNAIUC";
        let ioc = get_ioc(text);
        assert!(ioc < 1000.0);
    }
}