//! Maximum Index of Coincidence (MIC) Test
//!
//! This module implements the Maximum Index of Coincidence statistical test,
//! which finds the maximum IoC value when the text is split into different periods.

use crate::statistical_tests::utils::convert_string;

/// Calculates the Maximum Index of Coincidence (MIC) for the given text
///
/// The Maximum Index of Coincidence is found by calculating the IoC for different
/// period sizes and finding the maximum value. This is useful for identifying
/// polyalphabetic ciphers and determining their key length.
///
/// # Arguments
///
/// * `text` - The input text to analyze
///
/// # Returns
///
/// The Maximum Index of Coincidence value
///
/// # Examples
///
/// ```
/// use cipher_identifier::statistical_tests::mic::get_max_periodic_ic;
///
/// let text = "HELLOWORLD";
/// let mic = get_max_periodic_ic(text);
/// assert!(mic > 0.0);
/// ```
pub fn get_max_periodic_ic(text: &str) -> f64 {
    let data = convert_string(text);
    
    if data.len() < 2 {
        return 0.0;
    }
    
    let max_period = std::cmp::min(10, data.len() / 2);
    let mut max_ic = 0.0;
    
    // Try different period sizes
    for period in 1..=max_period {
        let ic = calculate_periodic_ic(&data, period);
        if ic > max_ic {
            max_ic = ic;
        }
    }
    
    max_ic
}

/// Calculates the Index of Coincidence for a specific period size
///
/// # Arguments
///
/// * `data` - The numeric representation of the text
/// * `period` - The period size to use
///
/// # Returns
///
/// The Index of Coincidence for the specified period
fn calculate_periodic_ic(data: &[usize], period: usize) -> f64 {
    let cipher_symbols = "ABCDEFGHIJKLMNOPQRSTUVWXYZ#0123456789";
    let num_symbols = cipher_symbols.len();
    
    let mut total_ic = 0.0;
    let mut valid_columns = 0;
    
    // Split the text into columns based on the period
    for i in 0..period {
        let mut column = Vec::new();
        let mut j = i;
        
        while j < data.len() {
            column.push(data[j]);
            j += period;
        }
        
        if column.len() > 1 {
            // Calculate IoC for this column
            let mut counts = vec![0; num_symbols];
            
            for &c in &column {
                counts[c] += 1;
            }
            
            let mut sum = 0.0;
            for count in counts {
                sum += count as f64 * (count as f64 - 1.0);
            }
            
            let column_len = column.len() as f64;
            let ic = sum / (column_len * (column_len - 1.0));
            
            total_ic += ic;
            valid_columns += 1;
        }
    }
    
    if valid_columns > 0 {
        (total_ic / valid_columns as f64) * 1000.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mic_vigenere() {
        // Vigenere cipher typically has a higher MIC than IoC
        let text = "LXFOPVEFRNHR"; // "HELLOWORLD" encrypted with Vigenere key "KEY"
        let mic = get_max_periodic_ic(text);
        let ioc = calculate_periodic_ic(&convert_string(text), 1);
        assert!(mic > ioc);
    }

    #[test]
    fn test_mic_short_text() {
        let text = "AB";
        let mic = get_max_periodic_ic(text);
        assert_eq!(mic, 0.0);
    }
}