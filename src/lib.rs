//! Cipher Identifier Library
//!
//! A library for identifying classical ciphers based on statistical analysis of ciphertext.
//! It calculates various statistical metrics and compares them against known patterns
//! for different cipher types to determine the most likely cipher used.

pub mod cipher_analyzer;
pub mod identify_cipher;
pub mod statistical_tests;
pub mod models;
pub mod benchmark;

/// Re-export main types for convenience
pub use cipher_analyzer::{CipherAnalyzer, CliArgs};
pub use identify_cipher::{get_cipher, identify_cipher};
pub use models::cipher_type::CipherType;