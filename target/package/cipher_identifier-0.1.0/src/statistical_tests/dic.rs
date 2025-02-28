//! Digraph Index of Coincidence (DIC) Test
//!
//! This module implements the Digraph Index of Coincidence statistical test,
//! which measures the frequency of digraphs (pairs of characters) in the text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Digraph Index of Coincidence (DIC) for the given text
///
/// The Digraph Index of Coincidence measures the frequency of digraphs (pairs of characters)
/// in the text. It's useful for distinguishing between different types of ciphers,
/// especially those that operate on pairs of characters.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Digraph Index of Coincidence value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::dic::get_dic;
///
/// let text = "HELLOWORLD";
/// let dic = get_dic(text);
/// assert!(dic > 0.0);
/// ```
pub fn get_dic(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    // Count digraph frequencies
    let mut digraph_counts = HashMap::new();
    let mut total_digraphs = 0;
    
    for i in 0..(data.len() - 1) {
        let digraph = (data[i], data[i + 1]);
        *digraph_counts.entry(digraph).or_insert(0) += 1;
        total_digraphs += 1;
    }
    
    // Calculate DIC
    let mut sum = 0.0;
    for &count in digraph_counts.values() {
        sum += count as f64 * (count as f64 - 1.0);
    }
    
    if total_digraphs <= 1 {
        return 0.0;
    }
    
    let dic = sum / (total_digraphs as f64 * (total_digraphs as f64 - 1.0));
    dic * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dic_repeated_digraphs() {
        // Text with repeated digraphs should have high DIC
        let text = "ABABABABAB";
        let dic = get_dic(text);
        assert!(dic > 100.0); // High DIC value expected
    }

    #[test]
    fn test_dic_varied_digraphs() {
        // Text with varied digraphs should have lower DIC
        let text = "ABCDEFGHIJ";
        let dic = get_dic(text);
        assert!(dic < 100.0); // Lower DIC value expected
    }

    #[test]
    fn test_dic_short_text() {
        // Text with only one character should return 0
        let text = "A";
        let dic = get_dic(text);
        assert_eq!(dic, 0.0);
    }
}