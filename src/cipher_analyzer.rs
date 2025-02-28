//! Cipher Analyzer Module
//!
//! This module provides the CLI interface for analyzing ciphertexts and identifying cipher types.

use clap::Parser;
use colored::Colorize;
use prettytable::{Cell, Row, Table};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::identify_cipher;
use crate::models::cipher_type::{load_cipher_types, get_cipher_primary_type};
use crate::statistical_tests::{binary_random, ioc, shannon_entropy};

/// Struct representing the CipherAnalyzer which provides the CLI interface
pub struct CipherAnalyzer;

/// CLI arguments for the cipher analyzer
#[derive(Parser, Debug)]
#[command(
    name = "cipher_analyzer",
    about = "Analyzes ciphertext and identifies the most likely cipher types",
    long_about = "Cipher Analyzer will analyze your ciphertext and run advanced algorithms on it to determine the correct encryption."
)]
pub struct CliArgs {
    /// The ciphertext to analyze
    #[arg(short, long)]
    text: Option<String>,

    /// The top n most likely ciphers to display
    #[arg(short, long, default_value = "5")]
    number: usize,

    /// Highlight a specific cipher in the list
    #[arg(short, long)]
    cipher: Option<String>,

    /// Verbosity level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Input file containing ciphertext
    #[arg(short, long)]
    file: Option<PathBuf>,
}

impl CipherAnalyzer {
    /// Creates a new CipherAnalyzer
    ///
    /// # Examples
    ///
    /// ```
    /// use cipher_identifier::cipher_analyzer::CipherAnalyzer;
    ///
    /// let analyzer = CipherAnalyzer::new();
    /// ```
    pub fn new() -> Self {
        CipherAnalyzer
    }

    /// Runs the cipher analyzer with the given CLI arguments
    ///
    /// # Arguments
    ///
    /// * `args` - The CLI arguments
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cipher_identifier::cipher_analyzer::{CipherAnalyzer, CliArgs};
    /// use clap::Parser;
    ///
    /// let args = CliArgs::parse();
    /// let analyzer = CipherAnalyzer::new();
    /// analyzer.run(args);
    /// ```
    pub fn run(&self, args: CliArgs) -> Result<(), Box<dyn Error>> {
        // Get text from file or command line
        let text = if let Some(file_path) = args.file {
            let mut file = File::open(file_path)?;
            let mut text = String::new();
            file.read_to_string(&mut text)?;
            text
        } else if let Some(text) = args.text {
            text
        } else {
            return Err("Text input expected. Run with --help for usage information.".into());
        };

        // Preprocess text: remove whitespace and convert to uppercase
        let text = Regex::new(r"\s+")?.replace_all(&text, "").to_string().to_uppercase();

        // Display basic statistics
        self.display_basic_stats(&text);

        // Identify cipher
        self.identify_cipher(&text, args.number, args.cipher.as_deref())?;

        Ok(())
    }

    /// Displays basic statistics about the ciphertext
    ///
    /// # Arguments
    ///
    /// * `text` - The ciphertext to analyze
    ///
    /// # Examples
    ///
    /// ```
    /// use cipher_identifier::cipher_analyzer::CipherAnalyzer;
    ///
    /// let analyzer = CipherAnalyzer::new();
    /// analyzer.display_basic_stats("HELLOWORLD");
    /// ```
    pub fn display_basic_stats(&self, text: &str) {
        let text_length = text.len();
        let text_ioc = ioc::get_ioc(text);
        let text_entropy = shannon_entropy::get_shannon_entropy(text);
        let binary_random_test = binary_random::get_binary_random(text);
        
        let mut table = Table::new();
        table.set_titles(Row::new(vec![
            Cell::new("Stat").style_spec("Fc"),
            Cell::new("Value").style_spec("Fb"),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("Length"),
            Cell::new(&text_length.to_string()),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("Number of unique characters"),
            Cell::new(&text.chars().collect::<std::collections::HashSet<_>>().len().to_string()),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("Missing letters"),
            Cell::new(&self.find_missing_letters(text)),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("IoC"),
            Cell::new(&format!("{:.6}", text_ioc)),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("Shannon entropy"),
            Cell::new(&format!("{:.6}", text_entropy)),
        ]));
        
        table.add_row(Row::new(vec![
            Cell::new("Binary random test"),
            Cell::new(&binary_random_test),
        ]));
        
        println!("\n{}", "Basic stats".bold());
        table.printstd();
    }

    /// Identifies the most likely cipher types for the given ciphertext
    ///
    /// # Arguments
    ///
    /// * `text` - The ciphertext to analyze
    /// * `number` - The number of top results to display
    /// * `highlight` - Optional cipher type to highlight in the results
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use cipher_identifier::cipher_analyzer::CipherAnalyzer;
    ///
    /// let analyzer = CipherAnalyzer::new();
    /// analyzer.identify_cipher("HELLOWORLD", 5, None);
    /// ```
    pub fn identify_cipher(&self, text: &str, number: usize, highlight: Option<&str>) -> Result<(), Box<dyn Error>> {
        let scores = identify_cipher::identify_cipher(text, number, highlight);
        
        let mut table = Table::new();
        table.set_titles(Row::new(vec![
            Cell::new("Cipher").style_spec("Fc"),
            Cell::new("Score").style_spec("Fb"),
            Cell::new("Cipher type").style_spec("Fg"),
        ]));
        
        // Try to load cipher types for additional information
        let cipher_types = load_cipher_types("resources/cipher_types.json").ok();
        
        for (cipher, score) in scores {
            let cipher_type = if let Some(ref types) = cipher_types {
                get_cipher_primary_type(types, &cipher)
            } else {
                "unknown".to_string()
            };
            
            if Some(cipher.as_str()) == highlight {
                table.add_row(Row::new(vec![
                    Cell::new(&cipher).style_spec("Fm"),
                    Cell::new(&format!("{:.3}", score)).style_spec("Fm"),
                    Cell::new(&cipher_type).style_spec("Fm"),
                ]));
            } else {
                table.add_row(Row::new(vec![
                    Cell::new(&cipher),
                    Cell::new(&format!("{:.3}", score)),
                    Cell::new(&cipher_type),
                ]));
            }
        }
        
        println!("\n{} (lower is better)", format!("Top {} most likely ciphers", number).bold());
        table.printstd();
        
        Ok(())
    }

    /// Finds letters that are missing from the ciphertext
    ///
    /// # Arguments
    ///
    /// * `text` - The ciphertext to analyze
    ///
    /// # Returns
    ///
    /// A string containing the missing letters
    ///
    /// # Examples
    ///
    /// ```
    /// use cipher_identifier::cipher_analyzer::CipherAnalyzer;
    ///
    /// let analyzer = CipherAnalyzer::new();
    /// let missing = analyzer.find_missing_letters("HELLOWORLD");
    /// assert!(missing.contains('A'));
    /// assert!(!missing.contains('L'));
    /// ```
    pub fn find_missing_letters(&self, text: &str) -> String {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let text_chars: std::collections::HashSet<char> = text.chars().collect();
        
        alphabet
            .chars()
            .filter(|&c| !text_chars.contains(&c))
            .collect()
    }
}

/// Main function for the cipher analyzer CLI
///
/// # Examples
///
/// ```no_run
/// use cipher_identifier::cipher_analyzer::main;
///
/// main();
/// ```
pub fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let analyzer = CipherAnalyzer::new();
    analyzer.run(args)
}