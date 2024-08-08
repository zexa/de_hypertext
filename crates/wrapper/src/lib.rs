pub use de_hypertext_core::DeserializeError;
pub use de_hypertext_core::Deserializer;
pub use de_hypertext_macro::Deserialize;

/// Export required so that crates using de_hypertext would not have to
/// explicitly add scraper into their Cargo.toml.
pub extern crate scraper;
