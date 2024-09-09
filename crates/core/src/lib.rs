use core::fmt::Display;
use scraper::ElementRef;
use std::error::Error;

pub use scraper;

pub mod derive;

pub trait Deserializer: Sized {
    fn from_document(document: &ElementRef) -> Result<Self, DeserializeError>;

    fn from_html(html: &str) -> Result<Self, DeserializeError> {
        let html = scraper::Html::parse_fragment(html);
        let document = html.root_element();
        Self::from_document(&document)
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    BuildingSelectorFailed {
        struct_name: String,
        field: String,
        selector: String,
    },
    ElementNotFoud {
        struct_name: String,
        field: String,
        selector: String,
    },
    AttributeNotFound {
        struct_name: String,
        field: String,
        selector: Option<String>,
        attribute: String,
    },
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for DeserializeError {}
