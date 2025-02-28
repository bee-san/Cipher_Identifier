# cipher_identifier

A Rust library and CLI tool for identifying classical ciphers based on statistical analysis of ciphertext. It calculates various statistical metrics and compares them against known patterns for different cipher types to determine the most likely cipher used.

## Features

- Analyzes ciphertext using multiple statistical tests
- Identifies the most likely cipher from 58 different classical cipher types
- Provides detailed statistical information about the ciphertext
- Command-line interface for easy use
- Can be used as a library in other Rust projects

## Supported Ciphers

The tool can identify the following 58 classical cipher types:

| Cipher Types |  |  |  |
|-------------|-------------|-------------|-------------|
| 6x6bifid | 6x6playfair | Autokey | Bazeries |
| Beaufort | CONDI | Grandpre | Grandpre10x10 |
| Gromark | NihilistSub6x6 | Patristocrat | Quagmire I |
| Quagmire II | Quagmire III | Quagmire IV | Slidefair |
| Swagman | Variant | Vigenere | amsco |
| bifid | cadenus | checkerboard | cmBifid |
| columnar | compressocrat | digrafid | foursquare |
| fractionatedMorse | grille | homophonic | keyphrase |
| monomeDinome | morbit | myszkowski | nicodemus |
| nihilistSub | nihilistTramp | numberedKey | periodicGromark |
| phillips | playfair | pollux | porta |
| portax | progressiveKey | ragbaby | redefence |
| routeTramp | runningKey | sequenceTramp | seriatedPlayfair |
| simplesubstitution | syllabary | tridigital | trifid |
| trisquare | twosquare | | |

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/cipher_identifier.git
cd cipher_identifier

# Build the project
cargo build --release

# The binary will be available at target/release/cipher_identifier
```

## Usage

### Command Line Interface

```bash
# Analyze ciphertext provided directly
cipher_identifier --text "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG" --number 5

# Analyze ciphertext from a file
cipher_identifier --file path/to/ciphertext.txt --number 10

# Highlight a specific cipher in the results
cipher_identifier --text "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG" --cipher playfair
```

### Command Line Options

- `--text`, `-t`: The ciphertext to analyze
- `--file`, `-f`: Input file containing ciphertext
- `--number`, `-n`: The top n most likely ciphers to display (default: 5)
- `--cipher`, `-c`: Highlight a specific cipher in the list
- `--verbose`, `-v`: Increase verbosity level
- `--help`, `-h`: Display help information

## API Documentation

### Library Overview

The `cipher_identifier` library provides a comprehensive API for analyzing ciphertext and identifying classical ciphers. It can be integrated into other Rust projects to add cipher identification capabilities.

### Key Components

1. **Cipher Identification**: Core functionality to identify the most likely cipher type
2. **Statistical Tests**: Various algorithms to analyze text patterns
3. **Cipher Type Definitions**: Data structures with cipher metadata

### API Output Format

#### Cipher Identification

The main function `identify_cipher` returns a vector of `CipherScore` pairs, which are tuples of `(String, f64)` representing the cipher name and its score:

```rust
// Type definition
pub type CipherScore = (String, f64);

// Example output
[
    ("playfair", 1.234),       // Lower scores indicate better matches
    ("bifid", 2.345),
    ("columnar", 3.456),
    ("vigenere", 4.567),
    ("simplesubstitution", 5.678)
]
```

The scores represent the "distance" between the statistical properties of the input text and the expected properties of each cipher type. **Lower scores indicate better matches**.

#### Statistical Tests

The `get_all_stats` function returns a `HashMap<String, f64>` containing the results of all statistical tests:

```rust
// Example output
{
    "IoC": 1.78,               // Index of Coincidence
    "MIC": 48.9,               // Mutual Index of Coincidence
    "MKA": 64.9,               // Mean Kappa Test
    "DIC": 16.5,               // Digraphic Index of Coincidence
    "EDI": 17.4,               // Even Distribution Index
    "LR": 4.9,                 // Length Ratio
    "ROD": 48.1,               // Repeat Order Distribution
    "LDI": 279.1,              // Letter Distribution Index
    "SDD": 68.2,               // Standard Deviation Distribution
    "Shannon": 4.2,            // Shannon Entropy
    "BinaryRandom": 1.0        // Binary Random Test (1.0 = passed, 0.0 = failed)
}
```

#### Cipher Type Information

The `CipherType` struct provides metadata about each cipher type:

```rust
// Example output for a single cipher type
CipherType {
    types: ["substitution"],                  // Primary classification
    subtypes: ["polygraphic"],                // Secondary classification
    subtypes2: ["fractionation"],             // Tertiary classification
    table: ["polybius square"],               // Table or grid used
    size: "6x6",                              // Size of the cipher
    notes: "Uses a 6x6 grid instead of 5x5"   // Additional notes
}
```

### Using the API

#### Identifying Ciphers

```rust
use cipher_identifier::identify_cipher;

fn main() {
    let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    
    // Get the top 5 most likely ciphers
    let results = identify_cipher(text, 5, None);
    
    for (cipher, score) in results {
        println!("{}: {:.3}", cipher, score);
    }
}
```

#### Getting Statistical Information

```rust
use cipher_identifier::statistical_tests::all_stats::get_all_stats;

fn main() {
    let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    
    // Get all statistical test results
    let stats = get_all_stats(text);
    
    for (test, value) in stats {
        println!("{}: {:.3}", test, value);
    }
}
```

#### Using the CipherAnalyzer

```rust
use cipher_identifier::CipherAnalyzer;

fn main() {
    let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    let analyzer = CipherAnalyzer::new();
    
    // Display basic statistics
    analyzer.display_basic_stats(text);
    
    // Identify the cipher
    analyzer.identify_cipher(text, 5, None).unwrap();
}
```

#### Loading Cipher Type Definitions

```rust
use cipher_identifier::models::cipher_type::{load_cipher_types, get_cipher_primary_type};

fn main() {
    // Load cipher type definitions
    let cipher_types = load_cipher_types("resources/cipher_types.json").unwrap();
    
    // Get the primary type of a specific cipher
    let primary_type = get_cipher_primary_type(&cipher_types, "playfair");
    println!("Primary type of playfair: {}", primary_type);
}
```

### Integration Example

Here's a complete example of how to integrate the cipher_identifier library into another project:

```rust
use cipher_identifier::{
    identify_cipher,
    statistical_tests::all_stats::get_all_stats,
    models::cipher_type::{load_cipher_types, get_cipher_primary_type},
};

fn analyze_ciphertext(text: &str) {
    // Step 1: Get statistical information
    let stats = get_all_stats(text);
    println!("Statistical Analysis:");
    println!("IoC: {:.6}", stats["IoC"]);
    println!("Shannon Entropy: {:.6}", stats["Shannon"]);
    
    // Step 2: Identify the cipher
    let results = identify_cipher(text, 5, None);
    
    // Step 3: Load cipher type definitions for additional information
    let cipher_types = load_cipher_types("resources/cipher_types.json").unwrap_or_default();
    
    // Step 4: Display results with additional information
    println!("\nMost Likely Ciphers:");
    for (i, (cipher, score)) in results.iter().enumerate() {
        let primary_type = get_cipher_primary_type(&cipher_types, cipher);
        println!("{}. {} (Score: {:.3}, Type: {})", 
                 i+1, cipher, score, primary_type);
    }
}

fn main() {
    let text = "THEQUICKBROWNFOXJUMPSOVERTHELAZYDOG";
    analyze_ciphertext(text);
}
```

## Statistical Tests

The tool uses the following statistical tests to analyze ciphertext:

- **IoC (Index of Coincidence)**: Measures the probability of two randomly selected letters being the same
- **MIC (Mutual Index of Coincidence)**: Measures the maximum periodic index of coincidence
- **MKA (Mean Kappa Test)**: Measures the average kappa value for the text
- **DIC (Digraphic Index of Coincidence)**: Measures the frequency of digraphs (pairs of letters)
- **EDI (Even Distribution Index)**: Measures how evenly distributed the digraphs are
- **LR (Length Ratio)**: Measures the ratio of unique n-grams to total possible n-grams
- **ROD (Repeat Order Distribution)**: Measures the distribution of repeated patterns
- **LDI (Letter Distribution Index)**: Measures how closely the letter distribution matches expected frequencies
- **SDD (Standard Deviation Distribution)**: Measures the standard deviation of letter frequencies
- **Shannon Entropy**: Measures the information content or randomness of the text
- **Binary Random Test**: Tests whether the text appears random when converted to binary

## Benchmarking

The library includes a benchmarking module to test the accuracy of the cipher identification algorithm:

```rust
use cipher_identifier::benchmark::run_benchmark;

fn main() {
    run_benchmark("path/to/test_data.json");
}
```

The test data should be a JSON file with each line containing a test case in the format:

```json
{"ciphertype": "playfair", "ciphertext": "HELLOWORLD"}
```

## License

[MIT License](LICENSE)
