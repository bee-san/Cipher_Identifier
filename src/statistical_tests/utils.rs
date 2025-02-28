//! Utility functions for statistical tests
//!
//! This module provides common utility functions used by various statistical tests.

/// Converts a string to a vector of numeric values representing characters
///
/// # Arguments
///
/// * `text` - The input text to convert
///
/// # Returns
///
/// A vector of numeric values where:
/// - A-Z are represented as 0-25
/// - '#' is represented as 26
/// - 0-9 are represented as 27-36
///
/// Characters not in the cipher_symbols set are ignored.
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::utils::convert_string;
///
/// let result = convert_string("ABC123");
/// assert_eq!(result, vec![0, 1, 2, 28, 29, 30]);
/// ```
pub fn convert_string(text: &str) -> Vec<usize> {
    let cipher_symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#0123456789";
    let mut num_code = Vec::new();
    
    for c in text.to_uppercase().chars() {
        // Replace Ø with 0 as in the Python code
        let c = if c == 'Ø' { '0' } else { c };
        
        if let Some(index) = cipher_symbols.find(c) {
            num_code.push(index);
        }
    }
    
    num_code
}

/// Checks if the data contains digits (characters with values > 26)
///
/// # Arguments
///
/// * `data` - Vector of numeric values representing characters
///
/// # Returns
///
/// "Y" if the data contains digits, "N" otherwise
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::utils::has_digits;
///
/// let data = vec![0, 1, 2, 30]; // Contains 30 which is > 26
/// assert_eq!(has_digits(&data), "Y");
///
/// let data = vec![0, 1, 2, 25]; // No values > 26
/// assert_eq!(has_digits(&data), "N");
/// ```
pub fn has_digits(data: &[usize]) -> String {
    for &c in data {
        if c > 26 {
            return "Y".to_string();
        }
    }
    "N".to_string()
}

/// Checks if the data contains the hash symbol (character with value 26)
///
/// # Arguments
///
/// * `data` - Vector of numeric values representing characters
///
/// # Returns
///
/// "Y" if the data contains the hash symbol, "N" otherwise
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::utils::has_hash;
///
/// let data = vec![0, 1, 26, 2]; // Contains 26 which represents '#'
/// assert_eq!(has_hash(&data), "Y");
///
/// let data = vec![0, 1, 2, 25]; // No value 26
/// assert_eq!(has_hash(&data), "N");
/// ```
pub fn has_hash(data: &[usize]) -> String {
    for &c in data {
        if c == 26 {
            return "Y".to_string();
        }
    }
    "N".to_string()
}
