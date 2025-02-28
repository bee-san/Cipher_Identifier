//! Statistical Tests Module
//!
//! This module contains various statistical tests used for cipher identification.

pub mod utils;
pub mod ioc;
pub mod mic;
pub mod mka;
pub mod dic;
pub mod edi;
pub mod lr;
pub mod rod;
pub mod ldi;
pub mod sdd;
pub mod binary_random;
pub mod shannon_entropy;
pub mod all_stats;

// Re-export commonly used functions
pub use all_stats::get_all_stats;
pub use ioc::get_ioc;
pub use shannon_entropy::get_shannon_entropy;
pub use binary_random::get_binary_random;