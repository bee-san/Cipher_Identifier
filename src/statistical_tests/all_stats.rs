//! All Stats Module
//!
//! This module combines all statistical tests and provides a function to run them all at once.

use std::collections::HashMap;

use crate::statistical_tests::{
    binary_random, dic, edi, ioc, ldi, lr, mic, mka, rod, sdd, shannon_entropy,
};

/// A type alias for the results of all statistical tests
pub type StatsResults = HashMap<String, f64>;

/// Runs all statistical tests on the given text and returns the results
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// A HashMap containing the results of all statistical tests
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::all_stats::get_all_stats;
///
/// let text = "HELLOWORLD";
/// let results = get_all_stats(text);
/// assert!(results.contains_key("IoC"));
/// assert!(results.contains_key("MIC"));
/// assert!(results.contains_key("MKA"));
/// ```
pub fn get_all_stats(text: &str) -> StatsResults {
    let mut results = HashMap::new();
    
    // Run all statistical tests
    results.insert("IoC".to_string(), ioc::get_ioc(text));
    results.insert("MIC".to_string(), mic::get_max_periodic_ic(text));
    results.insert("MKA".to_string(), mka::get_kappa(text));
    results.insert("DIC".to_string(), dic::get_dic(text));
    results.insert("EDI".to_string(), edi::get_even_dic(text));
    results.insert("LR".to_string(), lr::get_lr(text));
    results.insert("ROD".to_string(), rod::get_rod(text));
    results.insert("LDI".to_string(), ldi::get_ldi(text));
    results.insert("SDD".to_string(), sdd::get_sdd(text));
    
    // Additional tests that are not used in the cipher identification algorithm
    // but are useful for analysis
    results.insert("Shannon".to_string(), shannon_entropy::get_shannon_entropy(text));
    results.insert("BinaryRandom".to_string(), 
        if binary_random::get_binary_random(text) == "Y" { 1.0 } else { 0.0 });
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_stats() {
        let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
        let results = get_all_stats(text);
        
        // Check that all expected keys are present
        assert!(results.contains_key("IoC"));
        assert!(results.contains_key("MIC"));
        assert!(results.contains_key("MKA"));
        assert!(results.contains_key("DIC"));
        assert!(results.contains_key("EDI"));
        assert!(results.contains_key("LR"));
        assert!(results.contains_key("ROD"));
        assert!(results.contains_key("LDI"));
        assert!(results.contains_key("SDD"));
        assert!(results.contains_key("Shannon"));
        assert!(results.contains_key("BinaryRandom"));
    }
}