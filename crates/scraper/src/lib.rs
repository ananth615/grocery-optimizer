//! Scraper library for grocery-optimizer.
//!
//! Phase one: ingestion proof. The library exposes the config shape and
//! reader so the binary and tests share one parse path.

mod config;

pub use config::Config;
pub use config::read_config;
