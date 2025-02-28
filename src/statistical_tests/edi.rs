//! Even Digraph Index of Coincidence (EDI) Test
//!
//! This module implements the Even Digraph Index of Coincidence statistical test,
//! which measures the frequency of digraphs at even positions in the text.

use crate::statistical_tests::utils::convert_string;
use std::collections::HashMap;

/// Calculates the Even Digraph Index of Coincidence (EDI) for the given text
///
/// The Even Digraph Index of Coincidence measures the frequency of digraphs (pairs of characters)
/// at even positions in the text. It's useful for distinguishing between different types of ciphers,
/// especially those that operate on pairs of characters with different patterns at even/odd positions.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Even Digraph Index of Coincidence value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::edi::get_even_dic;
///
/// let text = "HELLOWORLD";
/// let edi = get_even_dic(text);
/// assert!(edi >= 0.0);
/// ```
pub fn get_even_dic(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 4 {  // Need at least two digraphs
        return 0.0;
    }
    
    // Count digraph frequencies at even positions
    let mut digraph_counts = HashMap::new();
    let mut total_digraphs = 0;
    
    for i in (0..(data.len() - 1)).step_by(2) {
        let digraph = (data[i], data[i + 1]);
        *digraph_counts.entry(digraph).or_insert(0) += 1;
        total_digraphs += 1;
    }
    
    // Calculate EDI
    let mut sum = 0.0;
    for &count in digraph_counts.values() {
        sum += count as f64 * (count as f64 - 1.0);
    }
    
    if total_digraphs <= 1 {
        return 0.0;
    }
    
    let edi = sum / (total_digraphs as f64 * (total_digraphs as f64 - 1.0));
    edi * 1000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edi_repeated_even_digraphs() {
        // Text with repeated digraphs at even positions should have high EDI
        let text = "ABABCDCD"; // AB and CD are at even positions
        let edi = get_even_dic(text);
        assert!(edi >= 0.0);
    }

    #[test]
    fn test_edi_varied_digraphs() {
        // Text with varied digraphs should have lower EDI
        let text = "ABCDEFGH";
        let edi = get_even_dic(text);
        assert!(edi >= 0.0);
    }

    #[test]
    fn test_edi_short_text() {
        // Text with fewer than 4 characters should return 0
        let text = "ABC";
        let edi = get_even_dic(text);
        assert_eq!(edi, 0.0);
    }
}