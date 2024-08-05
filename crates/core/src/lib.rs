use core::fmt::Display;
use scraper::ElementRef;
use std::error::Error;

pub trait Deserializer<T> {
    fn from_document(document: &ElementRef) -> Result<T, Box<dyn Error>>;

    fn from_html(html: &str) -> Result<T, Box<dyn Error>> {
        let html = scraper::Html::parse_document(html);
        let document = html.root_element();
        Self::from_document(&document)
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    ElementNotFoud {
        struct_name: String,
        field: String,
        selector: String,
    },
    AttributeNotFound {
        struct_name: String,
        field: String,
        selector: String,
        attribute: String,
    },
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{self:?}"))
    }
}

impl Error for DeserializeError {}
