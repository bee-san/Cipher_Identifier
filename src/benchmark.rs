//! Benchmark Module
//!
//! This module provides functionality for benchmarking the accuracy of the cipher identification algorithm.

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::identify_cipher;
use crate::statistical_tests::all_stats;

/// Represents a cipher test case
#[derive(Debug, Serialize, Deserialize)]
pub struct CipherTestCase {
    /// The type of cipher
    pub ciphertype: String,
    
    /// The ciphertext to analyze
    pub ciphertext: String,
}

/// Benchmarks the accuracy of the cipher identification algorithm
///
/// # Arguments
///
/// * `data_path` - Path to the JSON file containing test data
///
/// # Returns
///
/// A tuple containing the number of correct identifications and the total number of test cases
///
/// # Examples
///
/// ```no_run
/// use cipher_identifier::benchmark::benchmark;
///
/// let (correct, total) = benchmark("data/random_cipher_data.json").unwrap();
/// println!("{}/{} correct ({:.2}% accuracy)", correct, total, correct as f64 / total as f64 * 100.0);
/// ```
pub fn benchmark<P: AsRef<Path>>(data_path: P) -> Result<(usize, usize), Box<dyn Error>> {
    let file = File::open(data_path)?;
    let reader = BufReader::new(file);
    
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let test_case: CipherTestCase = serde_json::from_str(&line)?;
        data.push(test_case);
    }
    
    let mut correct = 0;
    
    // List of cipher types to check
    #[rustfmt::skip]
    let cipher_types = vec![
        "6x6bifid".to_string(), "6x6playfair".to_string(), "Autokey".to_string(), "Bazeries".to_string(), 
        "Beaufort".to_string(), "CONDI".to_string(), "Grandpre".to_string(), "Grandpre10x10".to_string(), 
        "Gromark".to_string(), "NihilistSub6x6".to_string(), "Patristocrat".to_string(), "Quagmire I".to_string(), 
        "Quagmire II".to_string(), "Quagmire III".to_string(), "Quagmire IV".to_string(), "Slidefair".to_string(), 
        "Swagman".to_string(), "Variant".to_string(), "Vigenere".to_string(), "amsco".to_string(), 
        "bifid".to_string(), "cadenus".to_string(), "checkerboard".to_string(), "cmBifid".to_string(), 
        "columnar".to_string(), "compressocrat".to_string(), "digrafid".to_string(), "foursquare".to_string(), 
        "fractionatedMorse".to_string(), "grille".to_string(), "homophonic".to_string(), "keyphrase".to_string(), 
        "monomeDinome".to_string(), "morbit".to_string(), "myszkowski".to_string(), "nicodemus".to_string(), 
        "nihilistSub".to_string(), "nihilistTramp".to_string(), "numberedKey".to_string(), "periodicGromark".to_string(), 
        "phillips".to_string(), "playfair".to_string(), "pollux".to_string(), "porta".to_string(), 
        "portax".to_string(), "progressiveKey".to_string(), "ragbaby".to_string(), "redefence".to_string(), 
        "routeTramp".to_string(), "runningKey".to_string(), "sequenceTramp".to_string(), "seriatedPlayfair".to_string(), 
        "simplesubstitution".to_string(), "syllabary".to_string(), "tridigital".to_string(), "trifid".to_string(), 
        "trisquare".to_string(), "twosquare".to_string()
    ];
    
    for item in &data {
        let stats = all_stats::get_all_stats(&item.ciphertext);
        
        // Extract the scores needed for cipher identification
        let scores = vec![
            stats["IoC"],
            stats["MIC"],
            stats["MKA"],
            stats["DIC"],
            stats["EDI"],
            stats["LR"],
            stats["ROD"],
            stats["LDI"],
            stats["SDD"],
        ];
        
        let num_dev = identify_cipher::get_cipher(&scores, &cipher_types);
        
        // Sort by score (lower is better)
        let mut num_dev = num_dev;
        num_dev.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Check if correct cipher is in top 5
        for (cipher, _) in num_dev.iter().take(5) {
            if cipher == &item.ciphertype {
                correct += 1;
                break;
            }
        }
    }
    
    Ok((correct, data.len()))
}

/// Runs the benchmark and prints the results
///
/// # Arguments
///
/// * `data_path` - Path to the JSON file containing test data
///
/// # Examples
///
/// ```no_run
/// use cipher_identifier::benchmark::run_benchmark;
///
/// run_benchmark("data/random_cipher_data.json");
/// ```
pub fn run_benchmark<P: AsRef<Path>>(data_path: P) {
    match benchmark(data_path) {
        Ok((correct, total)) => {
            println!("\n{}/{} correct", correct, total);
            println!("{:.2}% accuracy", correct as f64 / total as f64 * 100.0);
        }
        Err(e) => {
            eprintln!("Error running benchmark: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_benchmark_with_sample_data() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_data.json");
        
        // Create a sample test data file
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, r#"{{"ciphertype": "playfair", "ciphertext": "HELLOWORLD"}}"#).unwrap();
        writeln!(file, r#"{{"ciphertype": "simplesubstitution", "ciphertext": "ABCDEFGHIJKLMNOPQRSTUVWXYZ"}}"#).unwrap();
        
        // Run benchmark
        let (correct, total) = benchmark(&file_path).unwrap();
        
        // We don't care about the actual results, just that it runs without errors
        assert_eq!(total, 2);
    }
}