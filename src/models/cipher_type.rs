//! Cipher Type Model
//!
//! This module defines the CipherType struct which represents metadata for different cipher types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a cipher type with its metadata
///
/// # Examples
///
/// ```
/// use cipher_identifier::models::cipher_type::CipherType;
///
/// let cipher_type = CipherType {
///     types: vec!["substitution".to_string()],
///     subtypes: vec!["polygraphic".to_string()],
///     subtypes2: vec!["fractionation".to_string()],
///     table: vec!["polybius square".to_string()],
///     size: "6x6".to_string(),
///     notes: "The 6x6 Bifid cipher is a variant of the Bifid cipher, which uses a 6x6 grid instead of a 5x5 grid.".to_string(),
/// };
///
/// assert_eq!(cipher_type.types[0], "substitution");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherType {
    /// Primary classification of the cipher (e.g., "substitution", "transposition")
    #[serde(default)]
    pub types: Vec<String>,
    
    /// Secondary classification (e.g., "polygraphic", "polyalphabetic")
    #[serde(default)]
    pub subtypes: Vec<String>,
    
    /// Tertiary classification
    #[serde(default)]
    pub subtypes2: Vec<String>,
    
    /// Table or grid used by the cipher (e.g., "polybius square")
    #[serde(default)]
    pub table: Vec<String>,
    
    /// Size of the cipher (e.g., "6x6", "variable")
    #[serde(default)]
    pub size: String,
    
    /// Additional notes about the cipher
    #[serde(default)]
    pub notes: String,
}

/// A collection of cipher types indexed by name
pub type CipherTypes = HashMap<String, CipherType>;

/// Loads cipher types from a JSON file
///
/// # Arguments
///
/// * `path` - Path to the JSON file containing cipher type definitions
///
/// # Returns
///
/// A HashMap of cipher types indexed by name
///
/// # Examples
///
/// ```no_run
/// use cipher_identifier::models::cipher_type::load_cipher_types;
///
/// let cipher_types = load_cipher_types("resources/cipher_types.json").unwrap();
/// assert!(cipher_types.contains_key("6x6bifid"));
/// ```
pub fn load_cipher_types(path: &str) -> Result<CipherTypes, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let cipher_types: CipherTypes = serde_json::from_reader(file)?;
    Ok(cipher_types)
}

/// Gets the primary type of a cipher
///
/// # Arguments
///
/// * `cipher_types` - HashMap of cipher types
/// * `cipher` - Name of the cipher
///
/// # Returns
///
/// The primary type of the cipher, or "unknown" if not found
///
/// # Examples
///
/// ```no_run
/// use cipher_identifier::models::cipher_type::{load_cipher_types, get_cipher_primary_type};
///
/// let cipher_types = load_cipher_types("resources/cipher_types.json").unwrap();
/// let primary_type = get_cipher_primary_type(&cipher_types, "6x6bifid");
/// assert_eq!(primary_type, "substitution");
/// ```
pub fn get_cipher_primary_type(cipher_types: &CipherTypes, cipher: &str) -> String {
    cipher_types
        .get(cipher)
        .and_then(|ct| ct.types.first())
        .map(|t| t.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
